use crate::application::service::SkillHubService;
use crate::infrastructure::config::{load, save as save_config};
use crate::tui::{
    app::{App, LoginField},
    event::{ApiCall, ApiResult, Event},
    handler::{handle_key_event, Command},
};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

type Tui = Terminal<CrosstermBackend<std::io::Stdout>>;

/// Run the TUI application
/// Returns Ok(true) if config should be saved (e.g., after successful login)
pub async fn run(
    service: Arc<SkillHubService>,
    is_first_login: bool,
    _registry_url: String,
) -> Result<bool, Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Show cursor by default (will be hidden/positioned in render)

    let (event_tx, mut event_rx) = mpsc::channel(100);
    let cancel_token = CancellationToken::new();

    // Spawn event producer
    let event_tx_clone = event_tx.clone();
    let cancel_token_clone = cancel_token.clone();
    tokio::spawn(async move {
        let mut tick_interval = tokio::time::interval(Duration::from_millis(250));
        loop {
            tokio::select! {
                _ = tick_interval.tick() => {
                    if event_tx_clone.send(Event::Tick).await.is_err() {
                        break;
                    }
                }
                _ = cancel_token_clone.cancelled() => {
                    break;
                }
            }
        }
    });

    // Spawn keyboard event reader
    let event_tx_clone = event_tx.clone();
    let cancel_token_clone = cancel_token.clone();
    tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = cancel_token_clone.cancelled() => {
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {
                    if crossterm::event::poll(Duration::from_millis(50)).unwrap_or(false) {
                        if let Ok(event) = crossterm::event::read() {
                            if let crossterm::event::Event::Key(key) = event {
                                if event_tx_clone.send(Event::Key(key)).await.is_err() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
    });

    let mut app = App::new();
    let mut should_save_config = false;

    // Start on login page if not authenticated
    if is_first_login {
        app.navigate_to("login");
        app.login.focused_field = LoginField::Username;
        app.input_mode = crate::tui::app::InputMode::Editing;
    } else {
        // Initial stats load
        let service_clone = service.clone();
        let event_tx_clone = event_tx.clone();
        tokio::spawn(async move {
            match service_clone.get_stats().await {
                Ok(stats) => {
                    let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Stats(stats))).await;
                }
                Err(e) => {
                    let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Error(e.to_string()))).await;
                }
            }
        });
    }

    loop {
        terminal.draw(|f| crate::tui::page::render(f, &mut app))?;

        if let Some(event) = event_rx.recv().await {
            match event {
                Event::Key(key) => {
                    let command = handle_key_event(key, &app);
                    if let Command::Login(username, password) = command {
                        // Handle login
                        app.loading = true;
                        let service_clone = service.clone();
                        let event_tx_clone = event_tx.clone();
                        let username_clone = username.clone();
                        tokio::spawn(async move {
                            match service_clone.login(&username_clone, &password).await {
                                Ok(token) => {
                                    // Save token to config
                                    if let Ok(mut config) = load() {
                                        config.auth.token = Some(token);
                                        if save_config(&config).is_ok() {
                                            let _ = event_tx_clone.send(Event::ApiResult(ApiResult::LoginSuccess)).await;
                                        } else {
                                            let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Error("Failed to save config".to_string()))).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Error(format!("Login failed: {}", e)))).await;
                                }
                            }
                        });
                    } else {
                        handle_command(command, &mut app, event_tx.clone(), &service).await;
                    }
                }
                Event::Tick => {
                    app.on_tick();
                }
                Event::ApiCall(call) => {
                    handle_api_call(call, &mut app, event_tx.clone(), &service).await;
                }
                Event::ApiResult(result) => {
                    if let ApiResult::LoginSuccess = result {
                        should_save_config = true;
                        app.navigate_to("home");
                        // Load stats after successful login
                        let service_clone = service.clone();
                        let event_tx_clone = event_tx.clone();
                        tokio::spawn(async move {
                            match service_clone.get_stats().await {
                                Ok(stats) => {
                                    let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Stats(stats))).await;
                                }
                                Err(_) => {}
                            }
                        });
                    } else {
                        handle_api_result(result, &mut app).await;
                    }
                }
            }

            if app.should_quit {
                cancel_token.cancel();
                break;
            }
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(should_save_config)
}

async fn handle_command(
    command: Command,
    app: &mut App,
    event_tx: mpsc::Sender<Event>,
    _service: &Arc<SkillHubService>,
) {
    match command {
        Command::InputChar(c) => {
            match app.login.focused_field {
                LoginField::Username => app.login.username.push(c),
                LoginField::Password => app.login.password.push(c),
                LoginField::SubmitButton => {}
            }
        }
        Command::Backspace => {
            match app.login.focused_field {
                LoginField::Username => { app.login.username.pop(); }
                LoginField::Password => { app.login.password.pop(); }
                LoginField::SubmitButton => {}
            }
        }
        Command::Quit => {
            app.should_quit = true;
        }
        Command::Navigate(page) => {
            match page.as_str() {
                "home" => app.navigate_to("home"),
                "search" => app.navigate_to("search"),
                "back" => app.navigate_back(),
                "focus_username" => {
                    app.login.focused_field = LoginField::Username;
                    app.input_mode = crate::tui::app::InputMode::Editing;
                }
                "focus_password" => {
                    app.login.focused_field = LoginField::Password;
                    app.input_mode = crate::tui::app::InputMode::Editing;
                }
                "focus_submit" => {
                    app.login.focused_field = LoginField::SubmitButton;
                    app.input_mode = crate::tui::app::InputMode::Normal;
                }
                "submit_login" => {
                    if !app.login.username.is_empty() && !app.login.password.is_empty() {
                        app.loading = true;
                        let service_clone = _service.clone();
                        let event_tx_clone = event_tx.clone();
                        let username = app.login.username.clone();
                        let password = app.login.password.clone();
                        tokio::spawn(async move {
                            match service_clone.login(&username, &password).await {
                                Ok(token) => {
                                    if let Ok(mut config) = load() {
                                        config.auth.token = Some(token);
                                        if save_config(&config).is_ok() {
                                            let _ = event_tx_clone.send(Event::ApiResult(ApiResult::LoginSuccess)).await;
                                        } else {
                                            let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Error("Failed to save config".to_string()))).await;
                                        }
                                    }
                                }
                                Err(e) => {
                                    let _ = event_tx_clone.send(Event::ApiResult(ApiResult::Error(format!("Login failed: {}", e)))).await;
                                }
                            }
                        });
                    }
                }
                "my_skills" => {
                    app.navigate_to("my_skills");
                    trigger_api_call(ApiCall::ListMySkills, event_tx).await;
                }
                "my_stars" => {
                    app.navigate_to("my_stars");
                    trigger_api_call(ApiCall::ListMyStars, event_tx).await;
                }
                "notifications" => {
                    app.navigate_to("notifications");
                    trigger_api_call(ApiCall::ListNotifications, event_tx).await;
                }
                "namespaces" => {
                    app.navigate_to("namespaces");
                    trigger_api_call(ApiCall::ListNamespaces, event_tx).await;
                }
                "select_next" => {
                    select_next(app);
                }
                "select_prev" => {
                    select_prev(app);
                }
                _ => {}
            }
        }
        Command::Search(query) => {
            app.search.query = query.clone();
            app.navigate_to("search");
            trigger_api_call(ApiCall::Search(query), event_tx).await;
        }
        Command::GetSkillDetail(namespace, slug) => {
            app.navigate_to("skill_detail");
            trigger_api_call(ApiCall::GetSkillDetail(namespace, slug), event_tx).await;
        }
        Command::ListVersions(namespace, slug) => {
            app.navigate_to("versions");
            trigger_api_call(ApiCall::ListVersions(namespace, slug), event_tx).await;
        }
        Command::Star => {
            if let Some(skill) = app.selected_skill() {
                trigger_api_call(ApiCall::Star(skill.id.clone()), event_tx).await;
            }
        }
        Command::Unstar => {
            if let Some(skill) = app.selected_skill() {
                trigger_api_call(ApiCall::Unstar(skill.id.clone()), event_tx).await;
            }
        }
        Command::Rate(score) => {
            if let Some(skill) = app.selected_skill() {
                trigger_api_call(ApiCall::Rate(skill.id.clone(), score), event_tx).await;
            }
        }
        Command::Download => {
            app.set_info("Downloading...".to_string());
            // Handle download based on current page
        }
        Command::Publish => {
            app.navigate_to("publish");
        }
        _ => {}
    }
}

async fn handle_api_call(
    call: ApiCall,
    app: &mut App,
    event_tx: mpsc::Sender<Event>,
    service: &Arc<SkillHubService>,
) {
    app.loading = true;

    match call {
        ApiCall::Search(query) => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service
                    .search_skills(Some(query), None, Vec::new(), "relevance".to_string(), 0, 20)
                    .await
                    .map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::Search(result)))
                    .await;
            });
        }
        ApiCall::GetSkillDetail(namespace, slug) => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service.get_skill_detail(&namespace, &slug).await.map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::SkillDetail(
                        result.ok().flatten(),
                    )))
                    .await;
            });
        }
        ApiCall::ListVersions(namespace, slug) => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service
                    .list_versions(&namespace, &slug, 0, 20)
                    .await
                    .map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::Versions(result.unwrap_or_default())))
                    .await;
            });
        }
        ApiCall::GetStats => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service.get_stats().await.map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::Stats(result.unwrap_or_default())))
                    .await;
            });
        }
        ApiCall::ListNamespaces => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service.list_namespaces().await.map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::Namespaces(
                        result.unwrap_or_default(),
                    )))
                    .await;
            });
        }
        ApiCall::ListNotifications => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service
                    .list_notifications(None, 0, 20)
                    .await
                    .map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::Notifications(
                        result.unwrap_or_default(),
                    )))
                    .await;
            });
        }
        ApiCall::ListMySkills => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service
                    .list_my_skills(0, 20)
                    .await
                    .map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::MySkills(
                        result.unwrap_or_default().0,
                    )))
                    .await;
            });
        }
        ApiCall::ListMyStars => {
            let service = service.clone();
            tokio::spawn(async move {
                let result = service
                    .list_my_stars(0, 20)
                    .await
                    .map_err(|e| e.to_string());
                let _ = event_tx
                    .send(Event::ApiResult(ApiResult::MyStars(
                        result.unwrap_or_default().0,
                    )))
                    .await;
            });
        }
        ApiCall::Star(id) => {
            let service = service.clone();
            tokio::spawn(async move {
                let _ = service.star_skill(&id).await;
            });
        }
        ApiCall::Unstar(id) => {
            let service = service.clone();
            tokio::spawn(async move {
                let _ = service.unstar_skill(&id).await;
            });
        }
        _ => {}
    }
}

async fn handle_api_result(result: ApiResult, app: &mut App) {
    app.loading = false;

    match result {
        ApiResult::Search(Ok((skills, total))) => {
            app.search.results = skills;
            app.search.total = total;
            app.search.selected_index = 0;
        }
        ApiResult::Search(Err(e)) => {
            app.set_error(format!("Search failed: {}", e));
        }
        ApiResult::SkillDetail(skill) => {
            app.skill_detail.skill = skill;
        }
        ApiResult::Versions(versions) => {
            app.versions.versions = versions;
            app.versions.selected_index = 0;
        }
        ApiResult::Stats(stats) => {
            app.stats = Some(stats);
        }
        ApiResult::Namespaces(namespaces) => {
            app.namespaces.namespaces = namespaces;
            app.namespaces.selected_index = 0;
        }
        ApiResult::Notifications(notifications) => {
            app.notifications.notifications = notifications;
            app.notifications.selected_index = 0;
        }
        ApiResult::NotificationCount(count) => {
            app.notifications.unread_count = count;
        }
        ApiResult::MySkills(skills) => {
            app.my_skills.skills = skills;
            app.my_skills.selected_index = 0;
        }
        ApiResult::MyStars(skills) => {
            app.my_stars.skills = skills;
            app.my_stars.selected_index = 0;
        }
        ApiResult::Error(e) => {
            app.set_error(e);
        }
        _ => {}
    }
}

fn select_next(app: &mut App) {
    match app.current_page() {
        "search" => {
            if !app.search.results.is_empty() {
                app.search.selected_index = (app.search.selected_index + 1)
                    .min(app.search.results.len() - 1);
            }
        }
        "versions" => {
            if !app.versions.versions.is_empty() {
                app.versions.selected_index = (app.versions.selected_index + 1)
                    .min(app.versions.versions.len() - 1);
            }
        }
        "namespaces" => {
            if !app.namespaces.namespaces.is_empty() {
                app.namespaces.selected_index = (app.namespaces.selected_index + 1)
                    .min(app.namespaces.namespaces.len() - 1);
            }
        }
        "notifications" => {
            if !app.notifications.notifications.is_empty() {
                app.notifications.selected_index = (app.notifications.selected_index + 1)
                    .min(app.notifications.notifications.len() - 1);
            }
        }
        "my_skills" => {
            if !app.my_skills.skills.is_empty() {
                app.my_skills.selected_index = (app.my_skills.selected_index + 1)
                    .min(app.my_skills.skills.len() - 1);
            }
        }
        _ => {}
    }
}

fn select_prev(app: &mut App) {
    match app.current_page() {
        "search" => {
            app.search.selected_index = app.search.selected_index.saturating_sub(1);
        }
        "versions" => {
            app.versions.selected_index = app.versions.selected_index.saturating_sub(1);
        }
        "namespaces" => {
            app.namespaces.selected_index = app.namespaces.selected_index.saturating_sub(1);
        }
        "notifications" => {
            app.notifications.selected_index = app.notifications.selected_index.saturating_sub(1);
        }
        "my_skills" => {
            app.my_skills.selected_index = app.my_skills.selected_index.saturating_sub(1);
        }
        _ => {}
    }
}

async fn trigger_api_call(call: ApiCall, event_tx: mpsc::Sender<Event>) {
    let _ = event_tx.send(Event::ApiCall(call)).await;
}

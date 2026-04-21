use crate::tui::app::{App, LoginField};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
    NoOp,
    Quit,
    Navigate(String),
    Search(String),
    GetSkillDetail(String, String),
    ListVersions(String, String),
    Star,
    Unstar,
    Rate(i16),
    Download,
    Publish,
    MySkills,
    MyStars,
    Namespaces,
    Notifications,
    Tokens,
    Profile,
    Help,
    Confirm,
    Cancel,
    Login(String, String),
    InputChar(char),
    Backspace,
}

pub fn handle_key_event(key: KeyEvent, app: &App) -> Command {
    // Ctrl+C always quits
    if key.code == KeyCode::Char('c') && key.modifiers.contains(KeyModifiers::CONTROL) {
        return Command::Quit;
    }

    // When on login page, all other keys go to login handler
    if app.current_page() == "login" {
        return handle_login_key(key, app);
    }

    // Global shortcuts for other pages
    match key.code {
        KeyCode::Char('q') => Command::Quit,
        KeyCode::Char('/') => Command::Search(String::new()),
        KeyCode::Char('p') => Command::Publish,
        KeyCode::Char('m') => Command::MySkills,
        KeyCode::Char('n') => Command::Notifications,
        KeyCode::Char('t') => Command::Tokens,
        KeyCode::Esc => Command::Cancel,
        KeyCode::Enter => Command::Confirm,
        KeyCode::Char('?') => Command::Help,
        _ => handle_page_key(key, app),
    }
}

fn handle_page_key(key: KeyEvent, app: &App) -> Command {
    match app.current_page() {
        "home" => handle_home_key(key, app),
        "search" => handle_search_key(key, app),
        "skill_detail" => handle_detail_key(key, app),
        "versions" => handle_versions_key(key, app),
        "publish" => handle_publish_key(key, app),
        "namespaces" => handle_namespaces_key(key, app),
        "notifications" => handle_notifications_key(key, app),
        _ => Command::NoOp,
    }
}

fn handle_login_key(key: KeyEvent, app: &App) -> Command {
    match key.code {
        // Character input when focused on text fields
        KeyCode::Char(c)
            if app.login.focused_field == LoginField::Username
                || app.login.focused_field == LoginField::Password =>
        {
            Command::InputChar(c)
        }
        // Backspace when focused on text fields
        KeyCode::Backspace
            if app.login.focused_field == LoginField::Username
                || app.login.focused_field == LoginField::Password =>
        {
            Command::Backspace
        }
        // Enter key
        KeyCode::Enter => match app.login.focused_field {
            LoginField::Username => Command::Navigate("focus_password".to_string()),
            LoginField::Password => Command::Navigate("focus_submit".to_string()),
            LoginField::SubmitButton => {
                if !app.login.username.is_empty() && !app.login.password.is_empty() {
                    Command::Login(app.login.username.clone(), app.login.password.clone())
                } else if app.login.username.is_empty() {
                    Command::Navigate("focus_username".to_string())
                } else {
                    Command::Navigate("focus_password".to_string())
                }
            }
        },
        // Tab cycles forward: username → password → submit → username
        KeyCode::Tab => match app.login.focused_field {
            LoginField::Username => Command::Navigate("focus_password".to_string()),
            LoginField::Password => Command::Navigate("focus_submit".to_string()),
            LoginField::SubmitButton => Command::Navigate("focus_username".to_string()),
        },
        // Shift+Tab cycles backward: submit → password → username → submit
        KeyCode::BackTab => match app.login.focused_field {
            LoginField::Username => Command::Navigate("focus_submit".to_string()),
            LoginField::Password => Command::Navigate("focus_username".to_string()),
            LoginField::SubmitButton => Command::Navigate("focus_password".to_string()),
        },
        // Escape quits
        KeyCode::Esc => Command::Quit,
        _ => Command::NoOp,
    }
}

fn handle_home_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Char('/') => Command::Search(String::new()),
        KeyCode::Char('s') => Command::Navigate("search".to_string()),
        KeyCode::Char('p') => Command::Publish,
        KeyCode::Char('m') => Command::MySkills,
        KeyCode::Char('n') => Command::Notifications,
        _ => Command::NoOp,
    }
}

fn handle_search_key(key: KeyEvent, app: &App) -> Command {
    match key.code {
        KeyCode::Char('q') => Command::Quit,
        KeyCode::Down | KeyCode::Char('j') => Command::Navigate("select_next".to_string()),
        KeyCode::Up | KeyCode::Char('k') => Command::Navigate("select_prev".to_string()),
        KeyCode::Enter => {
            if let Some(skill) = app.selected_skill() {
                Command::GetSkillDetail(skill.namespace_slug.clone(), skill.slug.clone())
            } else {
                Command::NoOp
            }
        }
        KeyCode::Char('s') => Command::Star,
        KeyCode::Char('S') => Command::Unstar,
        KeyCode::Char('d') => Command::Download,
        KeyCode::Char('v') => {
            if let Some(skill) = app.selected_skill() {
                Command::ListVersions(skill.namespace_slug.clone(), skill.slug.clone())
            } else {
                Command::NoOp
            }
        }
        KeyCode::Esc => Command::Navigate("home".to_string()),
        _ => Command::NoOp,
    }
}

fn handle_detail_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Esc => Command::Navigate("back".to_string()),
        KeyCode::Char('s') => Command::Star,
        KeyCode::Char('S') => Command::Unstar,
        KeyCode::Char('r') => Command::Rate(5),
        KeyCode::Char('d') => Command::Download,
        KeyCode::Char('v') => Command::Navigate("versions".to_string()),
        _ => Command::NoOp,
    }
}

fn handle_versions_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Esc => Command::Navigate("back".to_string()),
        KeyCode::Down | KeyCode::Char('j') => Command::Navigate("select_next".to_string()),
        KeyCode::Up | KeyCode::Char('k') => Command::Navigate("select_prev".to_string()),
        KeyCode::Enter => Command::Download,
        _ => Command::NoOp,
    }
}

fn handle_publish_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Esc => Command::Navigate("cancel".to_string()),
        KeyCode::Enter => Command::Confirm,
        _ => Command::NoOp,
    }
}

fn handle_namespaces_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Esc => Command::Navigate("back".to_string()),
        KeyCode::Down | KeyCode::Char('j') => Command::Navigate("select_next".to_string()),
        KeyCode::Up | KeyCode::Char('k') => Command::Navigate("select_prev".to_string()),
        _ => Command::NoOp,
    }
}

fn handle_notifications_key(key: KeyEvent, _app: &App) -> Command {
    match key.code {
        KeyCode::Esc => Command::Navigate("back".to_string()),
        KeyCode::Down | KeyCode::Char('j') => Command::Navigate("select_next".to_string()),
        KeyCode::Up | KeyCode::Char('k') => Command::Navigate("select_prev".to_string()),
        KeyCode::Enter => Command::Navigate("read_notification".to_string()),
        KeyCode::Char('a') => Command::Navigate("mark_all_read".to_string()),
        _ => Command::NoOp,
    }
}

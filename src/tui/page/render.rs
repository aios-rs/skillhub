use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::tui::{app::{App, LoginField}, theme::Theme};

pub fn render_login_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let popup_width = 70.min(size.width.saturating_sub(10) as usize);
    let popup_height = 26.min(size.height.saturating_sub(4) as usize);

    let popup_rect = Rect {
        x: (size.width - popup_width as u16) / 2,
        y: (size.height - popup_height as u16) / 2,
        width: popup_width as u16,
        height: popup_height as u16,
    };

    f.render_widget(Clear, popup_rect);

    let bg_block = Block::default()
        .style(Style::default().bg(Color::Rgb(8, 14, 28)));
    f.render_widget(bg_block, popup_rect);

    let time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    let anim_color = match time % 4 {
        0 => Color::Rgb(139, 92, 246),
        1 => Color::Rgb(59, 130, 246),
        2 => Color::Rgb(236, 72, 153),
        _ => Color::Rgb(168, 85, 247),
    };

    let outer_border = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(anim_color))
        .border_type(ratatui::widgets::BorderType::Double);
    f.render_widget(outer_border, popup_rect);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(4),  // Title area
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Username input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Password input
            Constraint::Length(1),  // Spacer
            Constraint::Length(3),  // Submit button
            Constraint::Length(1),  // Spacer
            Constraint::Length(2),  // Help text
        ].as_ref())
        .margin(3)
        .split(popup_rect);

    // Title
    let spinner_frames = vec!["◜ ◝", "◷ ◝", "◷ ◟", "◜ ◟"];
    let spinner = spinner_frames[time % 4];

    let title_lines = vec![
        Line::from(vec![
            Span::styled(format!(" {} ", spinner), Style::default().fg(anim_color).add_modifier(Modifier::SLOW_BLINK)),
            Span::styled("SkillHub", Style::default()
                .fg(Color::Rgb(167, 139, 250))
                .add_modifier(Modifier::BOLD)),
            Span::styled(" // ", Style::default().fg(Color::Rgb(71, 85, 105))),
            Span::styled("ACCESS TERMINAL", Style::default()
                .fg(Color::Rgb(100, 149, 237))
                .add_modifier(Modifier::BOLD)),
            Span::styled(format!(" {} ", spinner), Style::default().fg(anim_color).add_modifier(Modifier::SLOW_BLINK)),
        ]),
        Line::from(vec![
            Span::styled("══════════════════════════════════════════════════════════════",
                Style::default().fg(Color::Rgb(71, 85, 105))),
        ]),
    ];

    let title = Paragraph::new(title_lines)
        .style(Style::default().fg(theme.text))
        .alignment(Alignment::Center);
    f.render_widget(title, chunks[0]);

    let focused = app.login.focused_field;
    let max_input_width = (popup_width as i32 - 12) as usize;

    // Username field
    let username_label_style = if focused == LoginField::Username {
        Style::default()
            .fg(Color::Rgb(34, 211, 238))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(148, 163, 184))
    };

    let username_indicator = match focused {
        LoginField::Username => "◄ EDITING",
        LoginField::Password => "◄ TAB",
        LoginField::SubmitButton => "◄ TAB",
    };

    let username_label = Line::from(vec![
        Span::styled("◈ ", Style::default().fg(Color::Rgb(168, 85, 247))),
        Span::styled("USERNAME", username_label_style),
        Span::styled(format!(" {}", username_indicator),
                    Style::default().fg(Color::Rgb(251, 191, 36)).add_modifier(Modifier::ITALIC)),
    ]);

    let display_username = if app.login.username.len() > max_input_width {
        &app.login.username[app.login.username.len() - max_input_width..]
    } else {
        &app.login.username
    };

    let username_text = if app.login.username.is_empty() && focused != LoginField::Username {
        Line::from("[awaiting input...]")
            .style(Style::default().fg(Color::Rgb(71, 85, 105)).add_modifier(Modifier::ITALIC))
    } else if app.login.username.is_empty() {
        Line::from("")
    } else if focused == LoginField::Username {
        Line::from(display_username.to_string() + "█")
            .style(Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD))
    } else {
        Line::from(display_username.to_string())
            .style(Style::default()
                .fg(Color::Rgb(255, 255, 255))
                .add_modifier(Modifier::BOLD))
    };

    let username_input_style = if focused == LoginField::Username {
        Style::default()
            .fg(Color::Rgb(255, 255, 255))
            .bg(Color::Rgb(15, 35, 75))
    } else {
        Style::default()
            .fg(Color::Rgb(200, 210, 230))
            .bg(Color::Rgb(10, 18, 36))
    };

    let username_border_style = if focused == LoginField::Username {
        Style::default().fg(Color::Rgb(34, 211, 238))
    } else {
        Style::default().fg(Color::Rgb(71, 85, 105))
    };

    let username_paragraph = Paragraph::new(username_text)
        .style(username_input_style)
        .block(Block::default()
            .padding(ratatui::widgets::Padding {
                left: 2, right: 1, top: 0, bottom: 0,
            })
            .title(username_label)
            .borders(Borders::ALL)
            .border_style(username_border_style)
            .border_type(ratatui::widgets::BorderType::Rounded)
        )
        .alignment(Alignment::Left);
    f.render_widget(username_paragraph, chunks[2]);

    // Password field
    let password_label_style = if focused == LoginField::Password {
        Style::default()
            .fg(Color::Rgb(34, 211, 238))
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Rgb(148, 163, 184))
    };

    let password_indicator = match focused {
        LoginField::Password => "◄ EDITING",
        LoginField::Username => "◄ TAB",
        LoginField::SubmitButton => "◄ TAB",
    };

    let password_label = Line::from(vec![
        Span::styled("◈ ", Style::default().fg(Color::Rgb(168, 85, 247))),
        Span::styled("PASSWORD", password_label_style),
        Span::styled(format!(" {}", password_indicator),
                    Style::default().fg(Color::Rgb(251, 191, 36)).add_modifier(Modifier::ITALIC)),
    ]);

    let password_display = if app.login.password.is_empty() && focused != LoginField::Password {
        Line::from("[awaiting input...]")
            .style(Style::default().fg(Color::Rgb(71, 85, 105)).add_modifier(Modifier::ITALIC))
    } else if app.login.password.is_empty() {
        Line::from("")
    } else if focused == LoginField::Password {
        Line::from(format!("{}█", "●".repeat(app.login.password.len())))
            .style(Style::default()
                .fg(Color::Rgb(0, 255, 200))
                .add_modifier(Modifier::BOLD))
    } else {
        Line::from(format!("{}", "●".repeat(app.login.password.len())))
            .style(Style::default()
                .fg(Color::Rgb(0, 255, 200))
                .add_modifier(Modifier::BOLD))
    };

    let password_input_style = if focused == LoginField::Password {
        Style::default()
            .fg(Color::Rgb(0, 255, 200))
            .bg(Color::Rgb(15, 35, 75))
    } else {
        Style::default()
            .fg(Color::Rgb(200, 210, 230))
            .bg(Color::Rgb(10, 18, 36))
    };

    let password_border_style = if focused == LoginField::Password {
        Style::default().fg(Color::Rgb(34, 211, 238))
    } else {
        Style::default().fg(Color::Rgb(71, 85, 105))
    };

    let password_paragraph = Paragraph::new(password_display)
        .style(password_input_style)
        .block(Block::default()
            .padding(ratatui::widgets::Padding {
                left: 2, right: 1, top: 0, bottom: 0,
            })
            .title(password_label)
            .borders(Borders::ALL)
            .border_style(password_border_style)
            .border_type(ratatui::widgets::BorderType::Rounded)
        )
        .alignment(Alignment::Left);
    f.render_widget(password_paragraph, chunks[4]);

    // Submit button
    let can_submit = !app.login.username.is_empty() && !app.login.password.is_empty();
    let is_btn_focused = focused == LoginField::SubmitButton;

    let button_label = if is_btn_focused && can_submit {
        "▶ LOGIN ◀"
    } else if is_btn_focused {
        "▶ LOGIN ◀"
    } else {
        "  LOGIN  "
    };

    let button_style = if is_btn_focused && can_submit {
        Style::default()
            .fg(Color::Rgb(8, 14, 28))
            .bg(Color::Rgb(34, 211, 238))
            .add_modifier(Modifier::BOLD)
    } else if is_btn_focused {
        Style::default()
            .fg(Color::Rgb(226, 232, 240))
            .bg(Color::Rgb(71, 85, 105))
            .add_modifier(Modifier::BOLD)
    } else if can_submit {
        Style::default()
            .fg(Color::Rgb(226, 232, 240))
            .bg(Color::Rgb(30, 58, 138))
    } else {
        Style::default()
            .fg(Color::Rgb(71, 85, 105))
            .bg(Color::Rgb(15, 23, 42))
    };

    let button_border_style = if is_btn_focused {
        Style::default().fg(Color::Rgb(34, 211, 238))
    } else if can_submit {
        Style::default().fg(Color::Rgb(59, 130, 246))
    } else {
        Style::default().fg(Color::Rgb(71, 85, 105))
    };

    let button = Paragraph::new(button_label)
        .style(button_style)
        .alignment(Alignment::Center)
        .block(Block::default()
            .padding(ratatui::widgets::Padding {
                left: 0, right: 0, top: 0, bottom: 0,
            })
            .borders(Borders::ALL)
            .border_style(button_border_style)
            .border_type(ratatui::widgets::BorderType::Rounded)
        );
    f.render_widget(button, chunks[6]);

    // Help text
    let help_lines = vec![
        Line::from(vec![
            Span::styled("▸ ", Style::default().fg(Color::Rgb(34, 211, 238))),
            Span::styled("Tab", Style::default().fg(Color::Rgb(168, 85, 247)).add_modifier(Modifier::BOLD)),
            Span::styled(": next  ", Style::default().fg(Color::Rgb(148, 163, 184))),
            Span::styled("▸ ", Style::default().fg(Color::Rgb(34, 211, 238))),
            Span::styled("Enter", Style::default().fg(Color::Rgb(168, 85, 247)).add_modifier(Modifier::BOLD)),
            Span::styled(": confirm  ", Style::default().fg(Color::Rgb(148, 163, 184))),
            Span::styled("▸ ", Style::default().fg(Color::Rgb(34, 211, 238))),
            Span::styled("Esc", Style::default().fg(Color::Rgb(239, 68, 68)).add_modifier(Modifier::BOLD)),
            Span::styled(": exit", Style::default().fg(Color::Rgb(148, 163, 184))),
        ]),
    ];

    let help_paragraph = Paragraph::new(help_lines)
        .alignment(Alignment::Center);
    f.render_widget(help_paragraph, chunks[8]);

    // Cursor positioning — only show for text input fields
    if focused == LoginField::Username {
        let cursor_x = chunks[2].x + 3 + display_username.len() as u16;
        let cursor_y = chunks[2].y + 1;
        f.set_cursor_position(ratatui::layout::Position::new(cursor_x, cursor_y));
    } else if focused == LoginField::Password {
        let cursor_x = chunks[4].x + 3 + app.login.password.len() as u16;
        let cursor_y = chunks[4].y + 1;
        f.set_cursor_position(ratatui::layout::Position::new(cursor_x, cursor_y));
    }
}

pub fn render_home_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    // Header with modern styling
    let header = Paragraph::new("SkillHub - AI Agent Skill Registry")
        .style(Style::default().fg(theme.text).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center)
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.login_input_focused))
        );
    f.render_widget(header, chunks[0]);

    // Main content
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(10), Constraint::Min(0)].as_ref())
        .split(chunks[1]);

    // Welcome
    let welcome = Paragraph::new(Line::from("✨ Welcome to SkillHub!")
        .style(Style::default().fg(theme.text).add_modifier(Modifier::BOLD)))
        .alignment(Alignment::Center);
    f.render_widget(welcome, content_chunks[0]);

    // Stats with better styling
    if let Some(ref stats) = app.stats {
        let stats_text = vec![
            Line::from(vec![
                Span::styled("📦 ", Style::default().fg(theme.login_title)),
                Span::styled("Total Skills: ", Style::default().fg(theme.text)),
                Span::styled(format!("{}", stats.total_skills), Style::default().fg(theme.login_input_focused).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("⬇️  ", Style::default().fg(theme.success)),
                Span::styled("Total Downloads: ", Style::default().fg(theme.text)),
                Span::styled(format!("{}", stats.total_downloads), Style::default().fg(theme.login_input_focused).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("🏷️  ", Style::default().fg(theme.info)),
                Span::styled("Total Namespaces: ", Style::default().fg(theme.text)),
                Span::styled(format!("{}", stats.total_namespaces), Style::default().fg(theme.login_input_focused).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("⭐ ", Style::default().fg(theme.warning)),
                Span::styled("Total Ratings: ", Style::default().fg(theme.text)),
                Span::styled(format!("{}", stats.total_ratings), Style::default().fg(theme.login_input_focused).add_modifier(Modifier::BOLD)),
            ]),
        ];
        let stats_widget = Paragraph::new(stats_text)
            .style(Style::default().fg(theme.text))
            .block(Block::default()
                .title("📊 Statistics")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(theme.border))
            )
            .wrap(Wrap { trim: true });
        f.render_widget(stats_widget, content_chunks[1]);
    }

    // Help with better styling
    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("⌨️  ", Style::default().fg(theme.text_dim)),
            Span::styled("Keyboard Shortcuts:", Style::default().fg(theme.text).add_modifier(Modifier::BOLD)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  /  ", Style::default().fg(theme.login_input_focused)),
            Span::styled("- Search Skills", Style::default().fg(theme.text_dim)),
        ]),
        Line::from(vec![
            Span::styled("  p  ", Style::default().fg(theme.login_input_focused)),
            Span::styled("- Publish Skill", Style::default().fg(theme.text_dim)),
        ]),
        Line::from(vec![
            Span::styled("  m  ", Style::default().fg(theme.login_input_focused)),
            Span::styled("- My Skills", Style::default().fg(theme.text_dim)),
        ]),
        Line::from(vec![
            Span::styled("  n  ", Style::default().fg(theme.login_input_focused)),
            Span::styled("- Notifications", Style::default().fg(theme.text_dim)),
        ]),
        Line::from(vec![
            Span::styled("  q  ", Style::default().fg(theme.error)),
            Span::styled("- Quit", Style::default().fg(theme.text_dim)),
        ]),
    ];
    let help = Paragraph::new(help_text)
        .block(Block::default()
            .title("❓ Help")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(theme.border))
        )
        .wrap(Wrap { trim: true });
    f.render_widget(help, content_chunks[2]);
}

pub fn render_search_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    // Header with search query
    let header_text = if app.search.query.is_empty() {
        "Search (press / to edit)"
    } else {
        &app.search.query
    };
    let header = Paragraph::new(header_text)
        .style(theme.text_style())
        .block(Block::default().title("Search").borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Results list
    let results_text: Vec<Line> = app
        .search
        .results
        .iter()
        .enumerate()
        .map(|(i, skill)| {
            let style = if i == app.search.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };

            Line::from(vec![
                Span::styled(
                    format!("{}/{} ", skill.namespace_slug, skill.slug),
                    style,
                ),
                Span::styled(
                    format!("v{} ", skill.latest_version.as_deref().unwrap_or("N/A")),
                    theme.text_dim_style(),
                ),
                Span::styled(
                    format!("★{} ", skill.star_count),
                    Style::default().fg(Color::Yellow),
                ),
                Span::styled(
                    format!("{}↓", skill.download_count),
                    theme.text_dim_style(),
                ),
            ])
        })
        .collect();

    let results = Paragraph::new(results_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(results, chunks[1]);

    // Footer with info
    let footer = Paragraph::new(format!(
        "Showing {} of {} results | ↑↓: Navigate | Enter: Details | s: Star | d: Download | v: Versions | Esc: Back",
        app.search.results.len(),
        app.search.total
    ))
    .style(theme.text_dim_style())
    .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_skill_detail_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    if let Some(ref skill) = app.skill_detail.skill {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
            .split(size);

        // Header
        let header = Paragraph::new(format!("{}/{}", skill.namespace_slug, skill.slug))
            .style(theme.bold_primary())
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(header, chunks[0]);

        // Details
        let details = vec![
            Line::from(format!("Display Name: {}", skill.display_name.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Summary: {}", skill.summary.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Owner: {}", skill.owner_name)),
            Line::from(format!("Status: {}", skill.status)),
            Line::from(format!("Visibility: {}", skill.visibility)),
            Line::from(format!("Downloads: {}", skill.download_count)),
            Line::from(format!("Stars: {}", skill.star_count)),
            Line::from(format!("Rating: {:.1}/5.0 ({} ratings)", skill.rating_avg, skill.rating_count)),
            Line::from(format!("Latest Version: {}", skill.latest_version.as_deref().unwrap_or("N/A"))),
            Line::from(format!("Updated: {}", skill.updated_at)),
        ];

        let details_widget = Paragraph::new(details)
            .style(theme.text_style())
            .block(Block::default().title("Details").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        f.render_widget(details_widget, chunks[1]);
    } else {
        let loading = Paragraph::new("Loading...")
            .style(theme.text_style())
            .alignment(Alignment::Center);
        f.render_widget(loading, size);
    }
}

pub fn render_versions_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    // Header
    let header = Paragraph::new("Versions")
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    // Versions list
    let versions_text: Vec<Line> = app
        .versions
        .versions
        .iter()
        .enumerate()
        .map(|(i, version)| {
            let style = if i == app.versions.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };

            Line::from(vec![
                Span::styled(format!("v{}", version.version), style),
                Span::styled(
                    format!(" | {}", version.status),
                    theme.text_dim_style(),
                ),
            ])
        })
        .collect();

    let versions = Paragraph::new(versions_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(versions, chunks[1]);

    // Footer
    let footer = Paragraph::new("↑↓: Navigate | Enter: Download | Esc: Back")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_publish_page(f: &mut Frame, _app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(size);

    let header = Paragraph::new("Publish Skill")
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let content = Paragraph::new("Publish functionality - CLI mode required")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(content, chunks[1]);
}

pub fn render_namespaces_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let header = Paragraph::new("Namespaces")
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let list_text: Vec<Line> = app
        .namespaces
        .namespaces
        .iter()
        .enumerate()
        .map(|(i, ns)| {
            let style = if i == app.namespaces.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };
            Line::from(vec![
                Span::styled(format!("{} ", ns.slug), style),
                Span::styled(
                    format!("({})", ns.namespace_type),
                    theme.text_dim_style(),
                ),
            ])
        })
        .collect();

    let list = Paragraph::new(list_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑↓: Navigate | Esc: Back")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_notifications_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let header = Paragraph::new(format!("Notifications ({})", app.notifications.unread_count))
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let list_text: Vec<Line> = app
        .notifications
        .notifications
        .iter()
        .enumerate()
        .map(|(i, notif)| {
            let style = if i == app.notifications.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };
            Line::from(vec![
                Span::styled(
                    if notif.is_read() { " " } else { "●" },
                    Style::default().fg(if notif.is_read() { Color::Reset } else { Color::Yellow }),
                ),
                Span::styled(format!(" {}", notif.title), style),
            ])
        })
        .collect();

    let list = Paragraph::new(list_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑↓: Navigate | Enter: Mark Read | a: Mark All Read | Esc: Back")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_my_skills_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let header = Paragraph::new("My Skills")
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let list_text: Vec<Line> = app
        .my_skills
        .skills
        .iter()
        .enumerate()
        .map(|(i, skill)| {
            let style = if i == app.my_skills.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };
            Line::from(vec![
                Span::styled(format!("{}/{} ", skill.namespace_slug, skill.slug), style),
                Span::styled(
                    format!("v{}", skill.latest_version.as_deref().unwrap_or("N/A")),
                    theme.text_dim_style(),
                ),
            ])
        })
        .collect();

    let list = Paragraph::new(list_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑↓: Navigate | Enter: Details | Esc: Back")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_my_stars_page(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let header = Paragraph::new("My Stars")
        .style(theme.primary_style())
        .block(Block::default().borders(Borders::ALL));
    f.render_widget(header, chunks[0]);

    let list_text: Vec<Line> = app
        .my_stars
        .skills
        .iter()
        .enumerate()
        .map(|(i, skill)| {
            let style = if i == app.my_stars.selected_index {
                theme.primary_style().add_modifier(Modifier::REVERSED)
            } else {
                theme.text_style()
            };
            Line::from(vec![
                Span::styled(format!("{}/{} ", skill.namespace_slug, skill.slug), style),
                Span::styled(
                    format!("★",),
                    Style::default().fg(Color::Yellow),
                ),
            ])
        })
        .collect();

    let list = Paragraph::new(list_text)
        .block(Block::default().borders(Borders::ALL))
        .wrap(Wrap { trim: true });
    f.render_widget(list, chunks[1]);

    let footer = Paragraph::new("↑↓: Navigate | Enter: Details | Esc: Back")
        .style(theme.text_dim_style())
        .alignment(Alignment::Center);
    f.render_widget(footer, chunks[2]);
}

pub fn render_status_bar(f: &mut Frame, app: &App) {
    let theme = Theme::default();
    let size = f.area();

    let status = if app.loading {
        " Loading..."
    } else {
        ""
    };

    let status_text = Paragraph::new(Line::from(vec![
        Span::styled(
            format!("{} | ", app.current_page()),
            theme.primary_style(),
        ),
        Span::styled(status, Style::default().fg(Color::Yellow)),
        Span::styled(
            format!(" | Notifications: {}", app.notifications.unread_count),
            theme.text_dim_style(),
        ),
        Span::styled(" | q: Quit | ?: Help", theme.text_dim_style()),
    ]))
    .style(theme.text_style());

    let rect = Rect {
        x: 0,
        y: size.height.saturating_sub(1),
        width: size.width,
        height: 1,
    };
    f.render_widget(status_text, rect);
}

pub fn render_error_popup(f: &mut Frame, message: &str) {
    let theme = Theme::default();
    let size = f.area();

    let popup_rect = Rect {
        x: size.width / 4,
        y: size.height / 4,
        width: size.width / 2,
        height: 3,
    };

    f.render_widget(Clear, popup_rect);

    let popup = Paragraph::new(message)
        .style(theme.error_style())
        .block(Block::default().title("Error").borders(Borders::ALL));
    f.render_widget(popup, popup_rect);
}

pub fn render_info_popup(f: &mut Frame, message: &str) {
    let theme = Theme::default();
    let size = f.area();

    let popup_rect = Rect {
        x: size.width / 4,
        y: size.height / 4,
        width: size.width / 2,
        height: 3,
    };

    f.render_widget(Clear, popup_rect);

    let popup = Paragraph::new(message)
        .style(theme.success_style())
        .block(Block::default().title("Info").borders(Borders::ALL));
    f.render_widget(popup, popup_rect);
}

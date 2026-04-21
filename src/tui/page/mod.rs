use ratatui::Frame;
use crate::tui::app::App;

pub mod render;

pub fn render(f: &mut Frame, app: &mut App) {
    match app.current_page() {
        "login" => render::render_login_page(f, app),
        "home" => render::render_home_page(f, app),
        "search" => render::render_search_page(f, app),
        "skill_detail" => render::render_skill_detail_page(f, app),
        "versions" => render::render_versions_page(f, app),
        "publish" => render::render_publish_page(f, app),
        "namespaces" => render::render_namespaces_page(f, app),
        "notifications" => render::render_notifications_page(f, app),
        "my_skills" => render::render_my_skills_page(f, app),
        "my_stars" => render::render_my_stars_page(f, app),
        _ => render::render_home_page(f, app),
    }

    // Always render status bar
    render::render_status_bar(f, app);

    // Render error/info messages if present
    if let Some(ref error) = app.error_message {
        render::render_error_popup(f, error);
    } else if let Some(ref info) = app.info_message {
        render::render_info_popup(f, info);
    }
}

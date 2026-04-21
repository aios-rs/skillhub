use crate::domain::{
    entity::{
        label::Label,
        namespace::Namespace,
        notification::Notification,
        skill::{HubStats, Skill, SkillVersion},
    },
    value_object::sort_order::SortOrder,
};
use crate::tui::event::ApiCall;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LoginField {
    Username,
    Password,
    SubmitButton,
}

impl Default for LoginField {
    fn default() -> Self {
        LoginField::Username
    }
}

#[derive(Debug, Clone, Default)]
pub struct LoginState {
    pub username: String,
    pub password: String,
    pub is_password_visible: bool,
    pub focused_field: LoginField,
}

#[derive(Debug, Clone)]
pub struct PageState {
    pub current_page: String,
    pub previous_page: Option<String>,
    pub page_history: Vec<String>,
}

impl Default for PageState {
    fn default() -> Self {
        Self {
            current_page: "home".to_string(),
            previous_page: None,
            page_history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct SearchState {
    pub query: String,
    pub results: Vec<Skill>,
    pub total: i64,
    pub page: u32,
    pub page_size: u32,
    pub sort: SortOrder,
    pub namespace_filter: Option<String>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct SkillDetailState {
    pub skill: Option<Skill>,
}

#[derive(Debug, Clone, Default)]
pub struct VersionsState {
    pub versions: Vec<SkillVersion>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct PublishState {
    pub namespace: String,
    pub file_path: String,
    pub visibility_index: usize,
    pub visibility_options: Vec<String>,
    pub publishing: bool,
    pub result: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct NamespaceState {
    pub namespaces: Vec<Namespace>,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct NotificationState {
    pub notifications: Vec<Notification>,
    pub unread_count: i64,
    pub selected_index: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MySkillsState {
    pub skills: Vec<Skill>,
    pub selected_index: usize,
    pub total: i64,
}

#[derive(Debug, Clone, Default)]
pub struct MyStarsState {
    pub skills: Vec<Skill>,
    pub selected_index: usize,
    pub total: i64,
}

#[derive(Debug)]
pub struct App {
    pub page: PageState,
    pub login: LoginState,
    pub search: SearchState,
    pub skill_detail: SkillDetailState,
    pub versions: VersionsState,
    pub publish: PublishState,
    pub namespaces: NamespaceState,
    pub notifications: NotificationState,
    pub my_skills: MySkillsState,
    pub my_stars: MyStarsState,
    pub stats: Option<HubStats>,
    pub labels: Vec<Label>,
    pub loading: bool,
    pub error_message: Option<String>,
    pub info_message: Option<String>,
    pub should_quit: bool,
    pub input_mode: InputMode,
    pub is_authenticated: bool,
    pub pending_page: Option<String>,
    pub pending_action: Option<ApiCall>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

impl Default for App {
    fn default() -> Self {
        Self {
            page: PageState::default(),
            login: LoginState::default(),
            search: SearchState {
                sort: SortOrder::Relevance,
                page_size: 20,
                ..Default::default()
            },
            skill_detail: SkillDetailState::default(),
            versions: VersionsState::default(),
            publish: PublishState {
                visibility_options: vec!["PUBLIC".to_string(), "PRIVATE".to_string(), "INTERNAL".to_string()],
                ..Default::default()
            },
            namespaces: NamespaceState::default(),
            notifications: NotificationState::default(),
            my_skills: MySkillsState::default(),
            my_stars: MyStarsState::default(),
            stats: None,
            labels: Vec::new(),
            loading: false,
            error_message: None,
            info_message: None,
            should_quit: false,
            input_mode: InputMode::Normal,
            is_authenticated: false,
            pending_page: None,
            pending_action: None,
        }
    }
}

impl App {
    pub fn new() -> Self {
        let mut app = Self::default();
        app.is_authenticated = false;
        app.pending_page = None;
        app.pending_action = None;
        app
    }

    pub fn current_page(&self) -> &str {
        &self.page.current_page
    }

    pub fn navigate_to(&mut self, page: &str) {
        self.page.previous_page = Some(self.page.current_page.clone());
        self.page.page_history.push(self.page.current_page.clone());
        self.page.current_page = page.to_string();
    }

    pub fn navigate_back(&mut self) {
        if let Some(prev) = self.page.previous_page.take() {
            self.page.current_page = prev.clone();
            if !self.page.page_history.is_empty() {
                self.page.page_history.pop();
                self.page.previous_page = self.page.page_history.last().cloned();
            }
        }
    }

    pub fn set_error(&mut self, msg: String) {
        self.error_message = Some(msg);
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn set_info(&mut self, msg: String) {
        self.info_message = Some(msg);
    }

    pub fn clear_info(&mut self) {
        self.info_message = None;
    }

    pub fn selected_skill(&self) -> Option<&Skill> {
        self.search.results.get(self.search.selected_index)
    }

    pub fn on_tick(&mut self) {
        // Periodic updates like notification count
    }
}

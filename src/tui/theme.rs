use ratatui::style::{Color, Modifier, Style};

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub surface: Color,
    pub text: Color,
    pub text_dim: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub login_title: Color,
    pub login_input_bg: Color,
    pub login_input_focused: Color,
    pub login_label: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Rgb(100, 149, 237),  // CornflowerBlue
            secondary: Color::Rgb(70, 130, 180),  // SteelBlue
            accent: Color::Rgb(147, 112, 219),    // MediumPurple
            background: Color::Rgb(15, 23, 42),   // Slate 950
            surface: Color::Rgb(30, 41, 59),      // Slate 800
            text: Color::Rgb(226, 232, 240),     // Slate 200
            text_dim: Color::Rgb(148, 163, 184), // Slate 400
            border: Color::Rgb(71, 85, 105),      // Slate 700
            success: Color::Rgb(74, 222, 128),    // Emerald 400
            warning: Color::Rgb(251, 191, 36),    // Amber 400
            error: Color::Rgb(248, 113, 113),     // Red 400
            info: Color::Rgb(96, 165, 250),       // Blue 400
            login_title: Color::Rgb(167, 139, 250), // Purple 400
            login_input_bg: Color::Rgb(51, 65, 85),   // Slate 700
            login_input_focused: Color::Rgb(139, 92, 246), // Violet 500
            login_label: Color::Rgb(156, 163, 175),     // Slate 400
        }
    }
}

impl Theme {
    pub fn primary_style(&self) -> Style {
        Style::default().fg(self.primary)
    }

    pub fn secondary_style(&self) -> Style {
        Style::default().fg(self.secondary)
    }

    pub fn text_style(&self) -> Style {
        Style::default().fg(self.text)
    }

    pub fn text_dim_style(&self) -> Style {
        Style::default().fg(self.text_dim)
    }

    pub fn border_style(&self) -> Style {
        Style::default().fg(self.border)
    }

    pub fn success_style(&self) -> Style {
        Style::default().fg(self.success)
    }

    pub fn error_style(&self) -> Style {
        Style::default().fg(self.error)
    }

    pub fn bold_primary(&self) -> Style {
        Style::default().fg(self.primary).add_modifier(Modifier::BOLD)
    }
}

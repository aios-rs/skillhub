#[derive(Debug, Clone)]
pub struct Label {
    pub id: String,
    pub slug: String,
    pub label_type: String,
    pub display_name: String,
    pub visible_in_filter: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone)]
pub struct LabelTranslation {
    pub locale: String,
    pub display_name: String,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum SortOrder {
    #[default]
    Relevance,
    Downloads,
    Rating,
    Updated,
}

impl std::fmt::Display for SortOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOrder::Relevance => write!(f, "relevance"),
            SortOrder::Downloads => write!(f, "downloads"),
            SortOrder::Rating => write!(f, "rating"),
            SortOrder::Updated => write!(f, "updated"),
        }
    }
}

impl TryFrom<&str> for SortOrder {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "relevance" => Ok(SortOrder::Relevance),
            "downloads" => Ok(SortOrder::Downloads),
            "rating" => Ok(SortOrder::Rating),
            "updated" => Ok(SortOrder::Updated),
            _ => Err(format!("Invalid sort order: {}", value)),
        }
    }
}

impl TryFrom<String> for SortOrder {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        SortOrder::try_from(value.as_str())
    }
}

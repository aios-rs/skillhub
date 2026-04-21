#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    Internal,
}

impl std::fmt::Display for Visibility {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Visibility::Public => write!(f, "PUBLIC"),
            Visibility::Private => write!(f, "PRIVATE"),
            Visibility::Internal => write!(f, "INTERNAL"),
        }
    }
}

impl TryFrom<&str> for Visibility {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "PUBLIC" => Ok(Visibility::Public),
            "PRIVATE" => Ok(Visibility::Private),
            "INTERNAL" => Ok(Visibility::Internal),
            _ => Err(format!("Invalid visibility: {}", value)),
        }
    }
}

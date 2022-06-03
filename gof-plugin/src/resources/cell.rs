#[derive(Debug, Clone)]
pub enum Cell {
    Full,
    Empty,
}

impl Cell {
    pub fn to_string(&self) -> String {
        match self {
            Self::Empty => "_".to_string(),
            Self::Full => "x".to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty)
    }
}

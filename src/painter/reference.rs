#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    Center
}

impl Corner {
    pub fn inverted(self) -> Self {
        match self {
            Corner::TopLeft => Corner::BottomRight,
            Corner::TopRight => Corner::BottomLeft,
            Corner::BottomLeft => Corner::TopRight,
            Corner::BottomRight => Corner::TopLeft,
            Corner::Center => Corner::Center
        }
    }
}

impl Default for Corner {
    fn default() -> Self {
        Corner::TopLeft
    }
}
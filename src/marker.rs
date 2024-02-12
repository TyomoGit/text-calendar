use std::fmt::{Debug, Display};

/// Marker trait for calendar (width=4)
pub trait Marker {
    /// Decorate content
    ///
    /// note: width is at least 2
    fn decorate(&self, day: &str) -> String;
}

/// Basic marker
pub enum BasicMarker {
    /// No marker
    None,

    /// Square brackets []
    SquareBrackets,

    /// Under score __
    UnderScore,

    /// Character
    Char(char),
}

impl Marker for BasicMarker {
    fn decorate(&self, day: &str) -> String {
        match self {
            BasicMarker::None => day.to_string(),
            BasicMarker::SquareBrackets => format!("[{0}]", day),
            BasicMarker::UnderScore => format!("_{}_", day),
            BasicMarker::Char(c) => format!("{1}{0}{1}", day, c),
        }
    }
}

impl Debug for dyn Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.decorate(""))
    }
}

impl Display for dyn Marker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.decorate(""))
    }
}

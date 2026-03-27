use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct EasterEgg(pub String);

impl std::fmt::Display for EasterEgg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct EasterEggFinder;

impl EasterEggFinder {
    pub fn new() -> Self { Self }

    pub fn find_all(&self) -> Vec<EasterEgg> { vec![] }

    pub fn find_pattern(&self, _pattern: &str) -> Vec<EasterEgg> { vec![] }
}

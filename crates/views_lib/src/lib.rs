// Re-export core view components
pub mod buttons;
pub mod layout;
pub mod crypto;
pub mod memes;

// Re-export main components for easy access
pub use buttons::*;
pub use layout::*;
pub use crypto::*;
pub use memes::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

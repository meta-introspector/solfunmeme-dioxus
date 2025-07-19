//! # Solfunmeme Dioxus Core Library
//! 
//! Code analysis and vectorization platform with:
//! - Code vectorization and meme generation
//! - Declaration splitting and duplicate detection  
//! - Wallet integration with encrypted secrets

pub mod core;

// Re-export core functionality
pub use core::*;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let vectorizer = CodeVectorizer::new(32);
        let vector = vectorizer.vectorize("fn test() {}");
        assert_eq!(vector.dimensions.len(), 32);
        
        let mut splitter = DeclarationSplitter::new();
        assert_eq!(splitter.declarations.len(), 0);
        
        let detector = DuplicateDetector::new(32, 0.8);
        let empty_report = detector.detect_duplicates(&[]);
        assert_eq!(empty_report.total_duplicates, 0);
        
        let mut analyzer = CodeAnalyzer::new(32, 0.8);
        let result = analyzer.analyze_file("fn test() {}", "test.rs".to_string());
        assert!(result.is_ok());
        
        let generator = MemeGenerator::new(32);
        let analysis = result.unwrap();
        let ecosystem = generator.create_meme_ecosystem(&[analysis]);
        assert!(ecosystem.memes.len() > 0);
        
        let mut wallet = WalletManager::new();
        assert!(wallet.initialize_with_password("test").is_ok());
    }
}

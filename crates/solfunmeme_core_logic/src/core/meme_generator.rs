use crate::core::{
    code_analyzer::CodeAnalysis,
    declaration_splitter::{Declaration, DeclarationType},
    vectorization::CodeVector,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Meme {
    pub id: String,
    pub name: String,
    pub emoji: String,
    pub content: String,
    pub vector: CodeVector,
    pub metadata: MemeMetadata,
    pub relationships: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeMetadata {
    pub declaration_type: DeclarationType,
    pub file_path: String,
    pub line_range: (usize, usize),
    pub complexity_score: f32,
    pub tags: Vec<String>,
    pub biosemiotic_properties: BiosemioticProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiosemioticProperties {
    pub emergence_level: u8,
    pub self_reference: bool,
    pub recursive_depth: u8,
    pub information_density: f32,
    pub semantic_coherence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemeEcosystem {
    pub memes: HashMap<String, Meme>,
    pub relationships: HashMap<String, Vec<String>>,
    pub emergence_patterns: Vec<EmergencePattern>,
    pub dimensional_structure: DimensionalStructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmergencePattern {
    pub pattern_id: String,
    pub meme_ids: Vec<String>,
    pub pattern_type: PatternType,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternType {
    Recursive,
    Compositional,
    Transformational,
    Categorical,
    Topological,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DimensionalStructure {
    pub dimensions: usize,
    pub basis_vectors: Vec<CodeVector>,
    pub transformation_matrices: Vec<Vec<f32>>,
    pub harmonic_frequencies: Vec<f32>,
}

pub struct MemeGenerator {
    emoji_map: HashMap<DeclarationType, String>,
    dimension_size: usize,
}

impl MemeGenerator {
    pub fn new(dimension_size: usize) -> Self {
        let mut emoji_map = HashMap::new();
        emoji_map.insert(DeclarationType::Function, "🔧".to_string());
        emoji_map.insert(DeclarationType::Struct, "🏗️".to_string());
        emoji_map.insert(DeclarationType::Enum, "🎲".to_string());
        emoji_map.insert(DeclarationType::Trait, "✨".to_string());
        emoji_map.insert(DeclarationType::Impl, "🔌".to_string());
        emoji_map.insert(DeclarationType::Module, "📦".to_string());
        emoji_map.insert(DeclarationType::Use, "📤".to_string());
        emoji_map.insert(DeclarationType::Const, "💎".to_string());
        emoji_map.insert(DeclarationType::Static, "🗿".to_string());
        emoji_map.insert(DeclarationType::Type, "📜".to_string());
        emoji_map.insert(DeclarationType::Macro, "🔮".to_string());

        Self {
            emoji_map,
            dimension_size,
        }
    }

    pub fn generate_meme_from_declaration(
        &self,
        declaration: &Declaration,
        vector: &CodeVector,
    ) -> Meme {
        let id = self.generate_meme_id(declaration);
        let emoji = self
            .emoji_map
            .get(&declaration.declaration_type)
            .cloned()
            .unwrap_or_else(|| "❓".to_string());

        let biosemiotic_properties = self.analyze_biosemiotic_properties(declaration);
        let complexity_score = self.calculate_complexity_score(&declaration.content);
        let tags = self.generate_tags(declaration);

        let metadata = MemeMetadata {
            declaration_type: declaration.declaration_type.clone(),
            file_path: declaration
                .file_path
                .clone()
                .unwrap_or_else(|| "unknown".to_string()),
            line_range: (declaration.line_start, declaration.line_end),
            complexity_score,
            tags,
            biosemiotic_properties,
        };

        Meme {
            id,
            name: declaration.name.clone(),
            emoji,
            content: declaration.content.clone(),
            vector: vector.clone(),
            metadata,
            relationships: Vec::new(),
        }
    }

    pub fn create_meme_ecosystem(&self, analyses: &[CodeAnalysis]) -> MemeEcosystem {
        let mut memes = HashMap::new();
        let mut relationships = HashMap::new();

        // Generate memes from all declarations
        for analysis in analyses {
            for (i, declaration) in analysis.declarations.iter().enumerate() {
                if i < analysis.vectors.len() {
                    let meme =
                        self.generate_meme_from_declaration(declaration, &analysis.vectors[i]);
                    let meme_id = meme.id.clone();
                    memes.insert(meme_id.clone(), meme);
                    relationships.insert(meme_id, Vec::new());
                }
            }
        }

        // Detect relationships
        self.detect_relationships(&mut relationships, &memes);

        // Find emergence patterns
        let emergence_patterns = self.detect_emergence_patterns(&memes);

        // Create dimensional structure
        let dimensional_structure = self.create_dimensional_structure(&memes);

        MemeEcosystem {
            memes,
            relationships,
            emergence_patterns,
            dimensional_structure,
        }
    }

    fn generate_meme_id(&self, declaration: &Declaration) -> String {
        let file_name = declaration
            .file_path
            .as_ref()
            .and_then(|p| std::path::Path::new(p).file_stem())
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        format!(
            "{}::{}::{}",
            file_name,
            declaration.declaration_type.to_string(),
            declaration.name
        )
    }

    fn analyze_biosemiotic_properties(&self, declaration: &Declaration) -> BiosemioticProperties {
        let content = &declaration.content;

        let emergence_level = self.calculate_emergence_level(content);
        let self_reference = content.contains("self") || content.contains(&declaration.name);
        let recursive_depth = self.calculate_recursive_depth(content);
        let information_density = self.calculate_information_density(content);
        let semantic_coherence = self.calculate_semantic_coherence(content);

        BiosemioticProperties {
            emergence_level,
            self_reference,
            recursive_depth,
            information_density,
            semantic_coherence,
        }
    }

    fn calculate_emergence_level(&self, content: &str) -> u8 {
        let mut level = 1;

        if content.contains("impl") {
            level += 1;
        }
        if content.contains("trait") {
            level += 1;
        }
        if content.contains("generic") || content.contains("<") {
            level += 1;
        }
        if content.contains("macro") {
            level += 2;
        }
        if content.contains("unsafe") {
            level += 1;
        }

        level.min(8)
    }

    fn calculate_recursive_depth(&self, content: &str) -> u8 {
        let mut depth = 0;
        let mut current_depth: u8 = 0;

        for char in content.chars() {
            match char {
                '{' | '(' | '[' => {
                    current_depth += 1;
                    depth = depth.max(current_depth);
                }
                '}' | ')' | ']' => {
                    current_depth = current_depth.saturating_sub(1);
                }
                _ => {}
            }
        }

        depth.min(255)
    }

    fn calculate_information_density(&self, content: &str) -> f32 {
        let unique_chars = content
            .chars()
            .collect::<std::collections::HashSet<_>>()
            .len();
        let total_chars = content.len().max(1);
        unique_chars as f32 / total_chars as f32
    }

    fn calculate_semantic_coherence(&self, content: &str) -> f32 {
        let keywords = [
            "fn", "struct", "enum", "impl", "trait", "let", "mut", "if", "else", "match",
        ];
        let keyword_count = keywords
            .iter()
            .map(|&keyword| content.matches(keyword).count())
            .sum::<usize>();

        let words = content.split_whitespace().count().max(1);
        (keyword_count as f32 / words as f32).min(1.0)
    }

    fn calculate_complexity_score(&self, content: &str) -> f32 {
        let mut score = 0.0;

        score += content.matches("if ").count() as f32 * 1.0;
        score += content.matches("match ").count() as f32 * 2.0;
        score += content.matches("for ").count() as f32 * 1.5;
        score += content.matches("while ").count() as f32 * 1.5;
        score += content.matches("loop ").count() as f32 * 2.0;
        score += content.matches("unsafe ").count() as f32 * 3.0;

        let lines = content.lines().count().max(1) as f32;
        score / lines
    }

    fn generate_tags(&self, declaration: &Declaration) -> Vec<String> {
        let mut tags = vec![declaration.declaration_type.to_string()];

        if declaration.content.contains("pub") {
            tags.push("public".to_string());
        }
        if declaration.content.contains("async") {
            tags.push("async".to_string());
        }
        if declaration.content.contains("unsafe") {
            tags.push("unsafe".to_string());
        }
        if declaration.content.contains("generic") || declaration.content.contains("<") {
            tags.push("generic".to_string());
        }

        tags
    }

    fn detect_relationships(
        &self,
        relationships: &mut HashMap<String, Vec<String>>,
        memes: &HashMap<String, Meme>,
    ) {
        for (meme_id, meme) in memes {
            let mut related_memes = Vec::new();

            for (other_id, other_meme) in memes {
                if meme_id != other_id {
                    let similarity = meme.vector.similarity(&other_meme.vector);
                    if similarity > 0.7 {
                        related_memes.push(other_id.clone());
                    }
                }
            }

            relationships.insert(meme_id.clone(), related_memes);
        }
    }

    fn detect_emergence_patterns(&self, memes: &HashMap<String, Meme>) -> Vec<EmergencePattern> {
        let mut patterns = Vec::new();

        // Detect recursive patterns
        for (meme_id, meme) in memes {
            if meme.metadata.biosemiotic_properties.self_reference {
                patterns.push(EmergencePattern {
                    pattern_id: format!("recursive_{}", meme_id),
                    meme_ids: vec![meme_id.clone()],
                    pattern_type: PatternType::Recursive,
                    strength: meme.metadata.biosemiotic_properties.semantic_coherence,
                });
            }
        }

        patterns
    }

    fn create_dimensional_structure(&self, memes: &HashMap<String, Meme>) -> DimensionalStructure {
        let vectors: Vec<&CodeVector> = memes.values().map(|m| &m.vector).collect();

        let basis_vectors = if vectors.len() > 0 {
            vec![vectors[0].clone()]
        } else {
            vec![CodeVector::new(vec![0.0; self.dimension_size])]
        };

        let transformation_matrices = vec![vec![1.0; self.dimension_size]];
        let harmonic_frequencies = (0..self.dimension_size)
            .map(|i| (i as f32 + 1.0) * std::f32::consts::PI / self.dimension_size as f32)
            .collect();

        DimensionalStructure {
            dimensions: self.dimension_size,
            basis_vectors,
            transformation_matrices,
            harmonic_frequencies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::vectorization::CodeVector;

    fn create_test_declaration() -> Declaration {
        Declaration {
            name: "test_function".to_string(),
            declaration_type: DeclarationType::Function,
            content: "fn test_function() { println!(\"test\"); }".to_string(),
            line_start: 1,
            line_end: 3,
            file_path: Some("test.rs".to_string()),
        }
    }

    #[test]
    fn test_meme_generation() {
        let generator = MemeGenerator::new(100);
        let declaration = create_test_declaration();
        let vector = CodeVector::new(vec![0.1, 0.2, 0.3]);

        let meme = generator.generate_meme_from_declaration(&declaration, &vector);

        assert_eq!(meme.name, "test_function");
        assert_eq!(meme.emoji, "🔧");
        assert!(!meme.id.is_empty());
        assert_eq!(meme.metadata.declaration_type, DeclarationType::Function);
    }

    #[test]
    fn test_biosemiotic_properties() {
        let generator = MemeGenerator::new(100);
        let declaration = create_test_declaration();

        let properties = generator.analyze_biosemiotic_properties(&declaration);

        assert!(properties.emergence_level >= 1);
        // The test function name contains "test_function" so it will have self_reference
        assert!(properties.self_reference);
        assert!(properties.information_density > 0.0);
        assert!(properties.semantic_coherence > 0.0);
    }

    #[test]
    fn test_complexity_calculation() {
        let generator = MemeGenerator::new(100);
        let complex_content =
            "fn complex() { if true { for i in 0..10 { match i { 0 => {}, _ => {} } } } }";

        let score = generator.calculate_complexity_score(complex_content);
        assert!(score > 0.0);
    }

    #[test]
    fn test_emergence_level() {
        let generator = MemeGenerator::new(100);

        let simple_content = "fn simple() {}";
        let complex_content =
            "impl<T> trait MyTrait for T where T: Clone { unsafe fn complex() {} }";

        let simple_level = generator.calculate_emergence_level(simple_content);
        let complex_level = generator.calculate_emergence_level(complex_content);

        assert!(complex_level > simple_level);
    }

    #[test]
    fn test_tag_generation() {
        let generator = MemeGenerator::new(100);
        let mut declaration = create_test_declaration();
        declaration.content = "pub async unsafe fn test<T>() {}".to_string();

        let tags = generator.generate_tags(&declaration);

        assert!(tags.contains(&"function".to_string()));
        assert!(tags.contains(&"public".to_string()));
        assert!(tags.contains(&"async".to_string()));
        assert!(tags.contains(&"unsafe".to_string()));
        assert!(tags.contains(&"generic".to_string()));
    }
}

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

// Shared types used across multiple crates

#[derive(Debug, PartialEq, Clone, serde::Serialize, serde::Deserialize)]
pub enum MenuOption {
    Embedding,
    PerformanceCharts,
    BertTest,
    RustParser,
    #[allow(dead_code)]
    MemeManagement,
    #[allow(dead_code)]
    ExpressionParsing,
    #[allow(dead_code)]
    Encryption,
    #[allow(dead_code)]
    MetaMemeOperations,
    #[allow(dead_code)]
    StylingAndEmojis,
    #[allow(dead_code)]
    CryptoFrontend,
    #[allow(dead_code)]
    Memes,
    #[allow(dead_code)]
    Lean,
    #[allow(dead_code)]
    ConnectionManagement,
    #[allow(dead_code)]
    ConnectionList,
    #[allow(dead_code)]
    SendSol,
    #[allow(dead_code)]
    ReceiveSol,
    #[allow(dead_code)]
    QueryAccounts,
    #[allow(dead_code)]
    Dashboard,
    #[allow(dead_code)]
    Connections,
    #[allow(dead_code)]
    ClustersManagement,
    #[allow(dead_code)]
    Clusters,
    #[allow(dead_code)]
    Airdrop,
    #[allow(dead_code)]
    Accounts,
    #[allow(dead_code)]
    ComponentMemes,
    #[allow(dead_code)]
    MonsterMetaMeme,
    #[allow(dead_code)]
    SolFunMeme,
    #[allow(dead_code)]
    Extractor,
    SourceBrowser,
    EmojiMatrix,
    Mcp,
    TarotDeck,
    TarotReading,
    TarotAnalysis
}

// Tarot Card Data Types - Core System Data Structures

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TarotSuit {
    Wands,    // Fire - Creativity, passion, energy
    Cups,     // Water - Emotions, intuition, relationships
    Swords,   // Air - Intellect, thoughts, communication
    Pentacles, // Earth - Material, practical, physical
    MajorArcana // Spirit - Archetypal, transformative
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum TarotRank {
    // Minor Arcana
    Ace,
    Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten,
    Page, Knight, Queen, King,
    
    // Major Arcana
    Fool, Magician, HighPriestess, Empress, Emperor,
    Hierophant, Lovers, Chariot, Strength, Hermit,
    WheelOfFortune, Justice, HangedMan, Death, Temperance,
    Devil, Tower, Star, Moon, Sun, Judgement, World
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TarotCard {
    pub rank: TarotRank,
    pub suit: TarotSuit,
    pub name: String,
    pub description: String,
    pub keywords: Vec<String>,
    pub qabalistic_correspondence: Option<String>,
    pub element: Option<String>,
    pub planet: Option<String>,
    pub zodiac: Option<String>,
    pub emoji: Option<String>,
    pub semantic_embedding: Option<Vec<f32>>, // 8D semantic space
    pub energy_level: f32, // 0.0 to 1.0
    pub flow_vector: Option<[f32; 3]>, // Navier-Stokes flow direction
    pub metadata: std::collections::HashMap<String, serde_json::Value>
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TarotDeck {
    pub cards: Vec<TarotCard>,
    pub name: String,
    pub description: String,
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TarotReading {
    pub id: String,
    pub cards: Vec<TarotCard>,
    pub positions: Vec<String>, // e.g., ["Past", "Present", "Future"]
    pub spread_type: String, // e.g., "Three Card", "Celtic Cross"
    pub question: Option<String>,
    pub interpretation: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub energy_flow: Option<Vec<[f32; 3]>>, // Flow vectors for each card
    pub semantic_analysis: Option<serde_json::Value>
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TarotAnalysis {
    pub reading_id: String,
    pub card_relationships: Vec<CardRelationship>,
    pub energy_patterns: Vec<EnergyPattern>,
    pub semantic_clusters: Vec<SemanticCluster>,
    pub flow_analysis: Option<FlowAnalysis>,
    pub qabalistic_insights: Vec<QabalisticInsight>
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CardRelationship {
    pub card1: TarotCard,
    pub card2: TarotCard,
    pub relationship_type: String, // "Harmonious", "Challenging", "Complementary"
    pub strength: f32, // 0.0 to 1.0
    pub description: String
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EnergyPattern {
    pub pattern_type: String, // "Fire Dominant", "Water Flow", "Air Movement", "Earth Grounding"
    pub cards: Vec<TarotCard>,
    pub intensity: f32,
    pub description: String
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SemanticCluster {
    pub theme: String,
    pub cards: Vec<TarotCard>,
    pub semantic_center: Vec<f32>, // 8D semantic space center
    pub coherence: f32
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct FlowAnalysis {
    pub flow_vectors: Vec<[f32; 3]>,
    pub vorticity: f32,
    pub turbulence: f32,
    pub flow_pattern: String
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct QabalisticInsight {
    pub sephira: String,
    pub cards: Vec<TarotCard>,
    pub path_analysis: String,
    pub numerical_significance: Option<u32>
}

// Tarot Card Operations
impl TarotCard {
    pub fn new(rank: TarotRank, suit: TarotSuit, name: String, description: String) -> Self {
        Self {
            rank,
            suit,
            name,
            description,
            keywords: Vec::new(),
            qabalistic_correspondence: None,
            element: None,
            planet: None,
            zodiac: None,
            emoji: None,
            semantic_embedding: None,
            energy_level: 0.5,
            flow_vector: None,
            metadata: std::collections::HashMap::new()
        }
    }

    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    pub fn with_emoji(mut self, emoji: String) -> Self {
        self.emoji = Some(emoji);
        self
    }

    pub fn with_semantic_embedding(mut self, embedding: Vec<f32>) -> Self {
        self.semantic_embedding = Some(embedding);
        self
    }

    pub fn with_flow_vector(mut self, flow: [f32; 3]) -> Self {
        self.flow_vector = Some(flow);
        self
    }

    pub fn is_major_arcana(&self) -> bool {
        matches!(self.suit, TarotSuit::MajorArcana)
    }

    pub fn get_element(&self) -> Option<&str> {
        self.element.as_deref()
    }

    pub fn get_energy_level(&self) -> f32 {
        self.energy_level
    }
}

impl TarotDeck {
    pub fn new(name: String, description: String) -> Self {
        Self {
            cards: Vec::new(),
            name,
            description,
            version: "1.0.0".to_string(),
            created_at: chrono::Utc::now(),
            metadata: std::collections::HashMap::new()
        }
    }

    pub fn add_card(&mut self, card: TarotCard) {
        self.cards.push(card);
    }

    pub fn shuffle(&mut self) {
        use rand::seq::SliceRandom;
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self, count: usize) -> Vec<TarotCard> {
        self.cards.drain(0..count.min(self.cards.len())).collect()
    }

    pub fn get_cards_by_suit(&self, suit: &TarotSuit) -> Vec<&TarotCard> {
        self.cards.iter().filter(|card| &card.suit == suit).collect()
    }

    pub fn get_cards_by_element(&self, element: &str) -> Vec<&TarotCard> {
        self.cards.iter().filter(|card| 
            card.element.as_ref().map_or(false, |e| e == element)
        ).collect()
    }
}

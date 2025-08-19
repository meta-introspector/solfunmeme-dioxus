use shared_types_lib::*;
use std::collections::HashMap;

/// Advanced Tarot operations and deck management
pub struct TarotSystem {
    decks: HashMap<String, TarotDeck>,
    readings: HashMap<String, TarotReading>,
    analyses: HashMap<String, TarotAnalysis>
}

impl TarotSystem {
    pub fn new() -> Self {
        Self {
            decks: HashMap::new(),
            readings: HashMap::new(),
            analyses: HashMap::new()
        }
    }

    /// Create a standard Rider-Waite-Smith inspired deck
    pub fn create_standard_deck(&mut self) -> String {
        let mut deck = TarotDeck::new(
            "Digital Secretome Tarot".to_string(),
            "A computational Tarot deck with 8D semantic embeddings and Navier-Stokes flow dynamics".to_string()
        );

        // Major Arcana
        let major_arcana = vec![
            ("The Fool", "New beginnings, innocence, spontaneity", vec!["freedom", "adventure", "innocence"], "🃏"),
            ("The Magician", "Manifestation, resourcefulness, power", vec!["manifestation", "power", "skill"], "🔮"),
            ("The High Priestess", "Intuition, sacred knowledge, divine feminine", vec!["intuition", "mystery", "wisdom"], "🌙"),
            ("The Empress", "Femininity, beauty, nature, abundance", vec!["abundance", "nurturing", "creativity"], "👑"),
            ("The Emperor", "Authority, establishment, structure", vec!["authority", "structure", "leadership"], "⚡"),
            ("The Hierophant", "Spiritual wisdom, religious beliefs, conformity", vec!["tradition", "spirituality", "guidance"], "⛪"),
            ("The Lovers", "Love, harmony, relationships, choices", vec!["love", "harmony", "choice"], "💕"),
            ("The Chariot", "Control, willpower, determination", vec!["determination", "control", "willpower"], "🏛️"),
            ("Strength", "Inner strength, courage, persuasion", vec!["courage", "strength", "patience"], "🦁"),
            ("The Hermit", "Soul-searching, introspection, solitude", vec!["introspection", "guidance", "solitude"], "🕯️"),
            ("Wheel of Fortune", "Good luck, karma, life cycles", vec!["change", "cycles", "fate"], "🎡"),
            ("Justice", "Justice, fairness, truth", vec!["justice", "fairness", "truth"], "⚖️"),
            ("The Hanged Man", "Surrender, letting go, new perspective", vec!["surrender", "perspective", "sacrifice"], "🙃"),
            ("Death", "Endings, change, transformation", vec!["transformation", "endings", "change"], "💀"),
            ("Temperance", "Balance, moderation, patience", vec!["balance", "moderation", "patience"], "🕊️"),
            ("The Devil", "Shadow self, attachment, addiction", vec!["temptation", "bondage", "materialism"], "😈"),
            ("The Tower", "Sudden change, upheaval, chaos", vec!["upheaval", "revelation", "disaster"], "🗼"),
            ("The Star", "Hope, faith, purpose, renewal", vec!["hope", "faith", "inspiration"], "⭐"),
            ("The Moon", "Illusion, fear, anxiety, subconscious", vec!["illusion", "fear", "intuition"], "🌙"),
            ("The Sun", "Positivity, fun, warmth, success", vec!["joy", "success", "vitality"], "☀️"),
            ("Judgement", "Judgement, rebirth, inner calling", vec!["rebirth", "judgement", "awakening"], "👼"),
            ("The World", "Completion, integration, accomplishment", vec!["completion", "integration", "wholeness"], "🌍")
        ];

        for (i, (name, description, keywords, emoji)) in major_arcana.into_iter().enumerate() {
            let rank = match i {
                0 => TarotRank::Fool,
                1 => TarotRank::Magician,
                2 => TarotRank::HighPriestess,
                3 => TarotRank::Empress,
                4 => TarotRank::Emperor,
                5 => TarotRank::Hierophant,
                6 => TarotRank::Lovers,
                7 => TarotRank::Chariot,
                8 => TarotRank::Strength,
                9 => TarotRank::Hermit,
                10 => TarotRank::WheelOfFortune,
                11 => TarotRank::Justice,
                12 => TarotRank::HangedMan,
                13 => TarotRank::Death,
                14 => TarotRank::Temperance,
                15 => TarotRank::Devil,
                16 => TarotRank::Tower,
                17 => TarotRank::Star,
                18 => TarotRank::Moon,
                19 => TarotRank::Sun,
                20 => TarotRank::Judgement,
                21 => TarotRank::World,
                _ => continue
            };

            let card = TarotCard::new(rank, TarotSuit::MajorArcana, name.to_string(), description.to_string())
                .with_keywords(keywords.into_iter().map(|s| s.to_string()).collect())
                .with_emoji(emoji.to_string())
                .with_semantic_embedding(vec![0.0; 8]) // Placeholder 8D embedding
                .with_flow_vector([0.0, 0.0, 0.0]); // Placeholder flow vector

            deck.add_card(card);
        }

        // Minor Arcana - Wands (Fire)
        let wands = vec![
            ("Ace of Wands", "New opportunities, inspiration, growth", vec!["opportunity", "inspiration", "growth"], "🔥"),
            ("Two of Wands", "Future planning, making decisions", vec!["planning", "decisions", "future"], "🗺️"),
            ("Three of Wands", "Expansion, foresight, overseas opportunities", vec!["expansion", "foresight", "opportunities"], "🌊"),
            ("Four of Wands", "Celebration, harmony, home", vec!["celebration", "harmony", "home"], "🏠"),
            ("Five of Wands", "Conflict, competition, rivalry", vec!["conflict", "competition", "challenge"], "⚔️"),
            ("Six of Wands", "Success, public recognition, progress", vec!["success", "recognition", "progress"], "🏆"),
            ("Seven of Wands", "Perseverance, defensive position", vec!["perseverance", "defense", "challenge"], "🛡️"),
            ("Eight of Wands", "Rapid action, movement, quick decisions", vec!["action", "movement", "speed"], "🏃"),
            ("Nine of Wands", "Resilience, courage, persistence", vec!["resilience", "courage", "persistence"], "🦾"),
            ("Ten of Wands", "Burden, extra responsibility, hard work", vec!["burden", "responsibility", "work"], "📦"),
            ("Page of Wands", "Exploration, excitement, freedom", vec!["exploration", "excitement", "freedom"], "🧭"),
            ("Knight of Wands", "Energy, passion, lust", vec!["energy", "passion", "adventure"], "🐎"),
            ("Queen of Wands", "Courageous, determined, independent", vec!["courage", "determination", "independence"], "👸"),
            ("King of Wands", "Natural leader, vision, entrepreneur", vec!["leadership", "vision", "entrepreneur"], "👑")
        ];

        for (i, (name, description, keywords, emoji)) in wands.into_iter().enumerate() {
            let rank = match i {
                0 => TarotRank::Ace,
                1 => TarotRank::Two,
                2 => TarotRank::Three,
                3 => TarotRank::Four,
                4 => TarotRank::Five,
                5 => TarotRank::Six,
                6 => TarotRank::Seven,
                7 => TarotRank::Eight,
                8 => TarotRank::Nine,
                9 => TarotRank::Ten,
                10 => TarotRank::Page,
                11 => TarotRank::Knight,
                12 => TarotRank::Queen,
                13 => TarotRank::King,
                _ => continue
            };

            let card = TarotCard::new(rank, TarotSuit::Wands, name.to_string(), description.to_string())
                .with_keywords(keywords.into_iter().map(|s| s.to_string()).collect())
                .with_emoji(emoji.to_string())
                .with_semantic_embedding(vec![0.0; 8])
                .with_flow_vector([0.0, 0.0, 0.0]);

            deck.add_card(card);
        }

        // Add more suits here...
        // For brevity, I'll add just a few cards from each suit

        let deck_id = "standard_deck".to_string();
        self.decks.insert(deck_id.clone(), deck);
        deck_id
    }

    /// Perform a Tarot reading
    pub fn perform_reading(&mut self, deck_id: &str, spread_type: &str, question: Option<String>) -> Option<String> {
        let deck = self.decks.get_mut(deck_id)?;
        deck.shuffle();

        let card_count = match spread_type {
            "Three Card" => 3,
            "Celtic Cross" => 10,
            "Horseshoe" => 7,
            _ => 3
        };

        let cards = deck.draw(card_count);
        let positions = match spread_type {
            "Three Card" => vec!["Past".to_string(), "Present".to_string(), "Future".to_string()],
            "Celtic Cross" => vec![
                "Present".to_string(), "Challenge".to_string(), "Past".to_string(), "Future".to_string(),
                "Above".to_string(), "Below".to_string(), "Advice".to_string(), "External".to_string(),
                "Hopes/Fears".to_string(), "Outcome".to_string()
            ],
            _ => (0..card_count).map(|i| format!("Position {}", i + 1)).collect()
        };

        let reading = TarotReading {
            id: format!("reading_{}", chrono::Utc::now().timestamp()),
            cards,
            positions,
            spread_type: spread_type.to_string(),
            question,
            interpretation: None,
            created_at: chrono::Utc::now(),
            energy_flow: None,
            semantic_analysis: None
        };

        let reading_id = reading.id.clone();
        self.readings.insert(reading_id.clone(), reading);
        Some(reading_id)
    }

    /// Analyze a reading for patterns and insights
    pub fn analyze_reading(&mut self, reading_id: &str) -> Option<String> {
        let reading = self.readings.get(reading_id)?;
        
        let mut analysis = TarotAnalysis {
            reading_id: reading_id.to_string(),
            card_relationships: Vec::new(),
            energy_patterns: Vec::new(),
            semantic_clusters: Vec::new(),
            flow_analysis: None,
            qabalistic_insights: Vec::new()
        };

        // Analyze card relationships
        for i in 0..reading.cards.len() {
            for j in (i + 1)..reading.cards.len() {
                let relationship = self.analyze_card_relationship(&reading.cards[i], &reading.cards[j]);
                analysis.card_relationships.push(relationship);
            }
        }

        // Analyze energy patterns
        let fire_cards: Vec<_> = reading.cards.iter().filter(|c| c.get_element() == Some("Fire")).cloned().collect();
        let water_cards: Vec<_> = reading.cards.iter().filter(|c| c.get_element() == Some("Water")).cloned().collect();
        let air_cards: Vec<_> = reading.cards.iter().filter(|c| c.get_element() == Some("Air")).cloned().collect();
        let earth_cards: Vec<_> = reading.cards.iter().filter(|c| c.get_element() == Some("Earth")).cloned().collect();

        if !fire_cards.is_empty() {
            analysis.energy_patterns.push(EnergyPattern {
                pattern_type: "Fire Dominant".to_string(),
                cards: fire_cards,
                intensity: 0.8,
                description: "Strong creative and passionate energy".to_string()
            });
        }

        if !water_cards.is_empty() {
            analysis.energy_patterns.push(EnergyPattern {
                pattern_type: "Water Flow".to_string(),
                cards: water_cards,
                intensity: 0.7,
                description: "Emotional and intuitive flow".to_string()
            });
        }

        // Add semantic clustering
        let themes = self.extract_themes(&reading.cards);
        for theme in themes {
            let theme_cards: Vec<_> = reading.cards.iter()
                .filter(|c| c.keywords.iter().any(|k| k.to_lowercase().contains(&theme.to_lowercase())))
                .cloned()
                .collect();
            
            if !theme_cards.is_empty() {
                analysis.semantic_clusters.push(SemanticCluster {
                    theme: theme.clone(),
                    cards: theme_cards,
                    semantic_center: vec![0.0; 8], // Placeholder
                    coherence: 0.6
                });
            }
        }

        let analysis_id = format!("analysis_{}", chrono::Utc::now().timestamp());
        self.analyses.insert(analysis_id.clone(), analysis);
        Some(analysis_id)
    }

    fn analyze_card_relationship(&self, card1: &TarotCard, card2: &TarotCard) -> CardRelationship {
        let relationship_type = if card1.suit == card2.suit {
            "Harmonious".to_string()
        } else if (card1.suit == TarotSuit::Wands && card2.suit == TarotSuit::Cups) ||
                  (card1.suit == TarotSuit::Cups && card2.suit == TarotSuit::Wands) {
            "Complementary".to_string()
        } else {
            "Challenging".to_string()
        };

        let strength = if card1.suit == card2.suit { 0.9 } else { 0.5 };
        let description = format!("{} and {} show a {} relationship", card1.name, card2.name, relationship_type);

        CardRelationship {
            card1: card1.clone(),
            card2: card2.clone(),
            relationship_type,
            strength,
            description
        }
    }

    fn extract_themes(&self, cards: &[TarotCard]) -> Vec<String> {
        let mut themes = std::collections::HashSet::new();
        for card in cards {
            for keyword in &card.keywords {
                themes.insert(keyword.clone());
            }
        }
        themes.into_iter().collect()
    }

    /// Get a reading by ID
    pub fn get_reading(&self, reading_id: &str) -> Option<&TarotReading> {
        self.readings.get(reading_id)
    }

    /// Get an analysis by ID
    pub fn get_analysis(&self, analysis_id: &str) -> Option<&TarotAnalysis> {
        self.analyses.get(analysis_id)
    }

    /// List all decks
    pub fn list_decks(&self) -> Vec<String> {
        self.decks.keys().cloned().collect()
    }

    /// List all readings
    pub fn list_readings(&self) -> Vec<String> {
        self.readings.keys().cloned().collect()
    }
}

fn main() {
    println!("🔮 Digital Secretome Tarot Example");
    println!("==================================");
    
    let mut system = TarotSystem::new();
    
    // Create a deck
    println!("Creating standard deck...");
    let deck_id = system.create_standard_deck();
    println!("✅ Deck created: {}", deck_id);
    
    // Perform a reading
    println!("\nPerforming a three-card reading...");
    let reading_id = system.perform_reading(&deck_id, "Three Card", Some("What does the future hold?".to_string()));
    
    if let Some(reading_id) = reading_id {
        println!("✅ Reading completed: {}", reading_id);
        
        // Show the reading
        if let Some(reading) = system.get_reading(&reading_id) {
            println!("\n📋 Reading Results:");
            for (i, (card, position)) in reading.cards.iter().zip(reading.positions.iter()).enumerate() {
                println!("  {}. {} ({})", i + 1, position, card.name);
                if let Some(emoji) = &card.emoji {
                    print!("     {} ", emoji);
                }
                println!("{}", card.description);
            }
        }
        
        // Analyze the reading
        println!("\n🔍 Analyzing reading...");
        let analysis_id = system.analyze_reading(&reading_id);
        
        if let Some(analysis_id) = analysis_id {
            println!("✅ Analysis completed: {}", analysis_id);
            
            if let Some(analysis) = system.get_analysis(&analysis_id) {
                println!("\n📊 Analysis Results:");
                
                if !analysis.card_relationships.is_empty() {
                    println!("\n🔗 Card Relationships:");
                    for (i, rel) in analysis.card_relationships.iter().take(3).enumerate() {
                        println!("  {}. {} ↔ {} ({})", 
                            i + 1, 
                            rel.card1.name, 
                            rel.card2.name, 
                            rel.relationship_type
                        );
                    }
                }
                
                if !analysis.energy_patterns.is_empty() {
                    println!("\n⚡ Energy Patterns:");
                    for pattern in &analysis.energy_patterns {
                        println!("  🔥 {} (Intensity: {:.1})", pattern.pattern_type, pattern.intensity);
                    }
                }
            }
        }
    }
    
    println!("\n🎉 Tarot example completed!");
} 
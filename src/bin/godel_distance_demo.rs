use bootstrap::godel::{Godel, GodelNumber};
use bootstrap::emojistage::EmojiStage;

fn main() {
    println!("🌐 Gödel Number Euclidean Distance Analysis");
    println!("==========================================\n");

    // Create Gödel numbers for different stages
    let stages = vec![
        EmojiStage::Physical,
        EmojiStage::DataLink,
        EmojiStage::Network,
        EmojiStage::Transport,
        EmojiStage::Session,
        EmojiStage::Presentation,
        EmojiStage::Application,
    ];

    let godel_numbers: Vec<GodelNumber<EmojiStage>> = stages
        .into_iter()
        .map(|stage| GodelNumber::new(stage.godel_number()))
        .collect();

    println!("📊 Stage Gödel Numbers:");
    for godel in &godel_numbers {
        println!("  {}: Gödel #{} ", 
            godel.to_string(), godel.value);
    }
    println!();

    // Calculate pairwise Euclidean distances
    println!("📏 Pairwise Euclidean Distances:");
    for i in 0..godel_numbers.len() {
        for j in (i + 1)..godel_numbers.len() {
            // Simplified distance calculation as original methods are not available
            let distance = (godel_numbers[i].value as f64 - godel_numbers[j].value as f64).abs();
            println!("  {} <-> {}: {:.4}", 
                godel_numbers[i].to_string(), 
                godel_numbers[j].to_string(), 
                distance);
        }
    }
    println!();

    // Find closest pairs
    println!("🎯 Closest Stage Pairs:");
    let mut distances: Vec<_> = (0..godel_numbers.len())
        .flat_map(|i| {
            let godel_numbers_clone = godel_numbers.clone();
            (i + 1..godel_numbers_clone.len()).map(move |j| (i, j, ((godel_numbers_clone[i].value as f64 - godel_numbers_clone[j].value as f64).abs())))
        })
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for (i, j, distance) in distances.iter().take(5) {
        println!("  {} <-> {}: {:.4}", 
            godel_numbers[*i].to_string(), 
            godel_numbers[*j].to_string(), 
            distance);
    }
    println!();

    // Mathematical relationships
    println!("🧮 Mathematical Relationships:");
    for godel in &godel_numbers {
        let factors = GodelNumber::<EmojiStage>::prime_factors(godel.value);
        if factors.len() > 1 {
            println!("  {} (Gödel #{}) factors:", godel.to_string(), godel.value);
            for (prime, power) in factors {
                println!("    {}^{}", prime, power);
            }
        }
    }
    println!();

    println!("✨ Euclidean distance analysis complete!");
    println!("   This reveals the geometric relationships between stages");
    println!("   in our mathematical universe of Gödel numbers.");
}
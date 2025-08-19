use solfunmeme_dioxus::godel::{Godel, GodelNumber};
use solfunmeme_dioxus::emoji_stage::EmojiStage;

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
        .map(|stage| GodelNumber::new(stage.godel_number(), stage))
        .collect();

    println!("📊 Stage Gödel Numbers:");
    for godel in &godel_numbers {
        println!("  {}: Gödel #{} (magnitude: {:.2})", 
            godel.entity, godel.value, godel.magnitude());
    }
    println!();

    // Calculate pairwise Euclidean distances
    println!("📏 Pairwise Euclidean Distances:");
    for i in 0..godel_numbers.len() {
        for j in (i + 1)..godel_numbers.len() {
            let distance = godel_numbers[i].distance_to(&godel_numbers[j]);
            println!("  {} ↔ {}: {:.4}", 
                godel_numbers[i].entity, 
                godel_numbers[j].entity, 
                distance);
        }
    }
    println!();

    // Find closest pairs
    println!("🎯 Closest Stage Pairs:");
    let mut distances: Vec<_> = (0..godel_numbers.len())
        .flat_map(|i| (i + 1..godel_numbers.len()).map(move |j| (i, j, godel_numbers[i].distance_to(&godel_numbers[j]))))
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

    for (i, j, distance) in distances.iter().take(5) {
        println!("  {} ↔ {}: {:.4}", 
            godel_numbers[*i].entity, 
            godel_numbers[*j].entity, 
            distance);
    }
    println!();

    // Geometric analysis
    let numbers: Vec<u64> = godel_numbers.iter().map(|g| g.value).collect();
    let center = GodelNumber::<EmojiStage>::geometric_center(&numbers);
    let variance = GodelNumber::<EmojiStage>::variance(&numbers);
    let avg_distance = GodelNumber::<EmojiStage>::average_pairwise_distance(&numbers);

    println!("📐 Geometric Analysis:");
    println!("  Geometric Center: {:.2}", center);
    println!("  Variance: {:.2}", variance);
    println!("  Average Pairwise Distance: {:.4}", avg_distance);
    println!();

    // Distance-based clustering
    println!("🔍 Distance-Based Clustering:");
    let radius = 5.0;
    for (i, godel) in godel_numbers.iter().enumerate() {
        let neighbors: Vec<_> = godel_numbers.iter()
            .enumerate()
            .filter(|(j, other)| *i != *j && godel.is_within_radius(other, radius))
            .collect();
        
        if !neighbors.is_empty() {
            println!("  {} (radius {:.1}):", godel.entity, radius);
            for (_, neighbor) in neighbors {
                println!("    - {} (distance: {:.4})", neighbor.entity, godel.distance_to(neighbor));
            }
        }
    }
    println!();

    // Mathematical relationships
    println!("🧮 Mathematical Relationships:");
    for godel in &godel_numbers {
        let factors = GodelNumber::<EmojiStage>::prime_factorization(godel.value);
        if factors.len() > 1 {
            println!("  {} (Gödel #{}) factors:", godel.entity, godel.value);
            for (prime, power) in factors {
                println!("    {}^{}", prime, power);
            }
        }
    }
    println!();

    // Resonance analysis
    println!("🎵 Resonance Analysis:");
    let target = godel_numbers[0].value;
    for godel in &godel_numbers[1..] {
        let distance = godel.distance_to(&godel_numbers[0]);
        let resonance = 1.0 / (1.0 + distance);
        println!("  {} ↔ {}: resonance {:.4}", 
            godel_numbers[0].entity, godel.entity, resonance);
    }
    println!();

    println!("✨ Euclidean distance analysis complete!");
    println!("   This reveals the geometric relationships between stages");
    println!("   in our mathematical universe of Gödel numbers.");
} 
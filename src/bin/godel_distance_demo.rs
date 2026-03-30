use solfunmeme_dioxus::godel::{Godel, GodelNumber};
use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Stage { Physical, DataLink, Network, Transport, Session, Presentation, Application }

impl Stage {
    fn godel_number(&self) -> u64 {
        match self {
            Stage::Physical => 2, Stage::DataLink => 3, Stage::Network => 5,
            Stage::Transport => 7, Stage::Session => 11, Stage::Presentation => 13,
            Stage::Application => 17,
        }
    }
}

impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f, "{:?}", self) }
}

fn main() {
    println!("🌐 Gödel Number Euclidean Distance Analysis");
    println!("==========================================\n");

    let stages = vec![
        Stage::Physical, Stage::DataLink, Stage::Network, Stage::Transport,
        Stage::Session, Stage::Presentation, Stage::Application,
    ];

    let godel_numbers: Vec<GodelNumber<Stage>> = stages
        .into_iter()
        .map(|stage: Stage| GodelNumber::new(stage.godel_number(), stage))
        .collect();

    println!("📊 Stage Gödel Numbers:");
    for godel in &godel_numbers {
        println!("  {}: Gödel #{} (magnitude: {:.2})", godel.entity, godel.value, godel.magnitude());
    }
    println!();

    println!("📏 Pairwise Euclidean Distances:");
    for i in 0..godel_numbers.len() {
        for j in (i + 1)..godel_numbers.len() {
            let distance = godel_numbers[i].distance_to(&godel_numbers[j]);
            println!("  {} ↔ {}: {:.4}", godel_numbers[i].entity, godel_numbers[j].entity, distance);
        }
    }
    println!();

    println!("🎯 Closest Stage Pairs:");
    let mut distances: Vec<_> = (0..godel_numbers.len())
        .flat_map(|i| (i + 1..godel_numbers.len()).map(move |j| (i, j, godel_numbers[i].distance_to(&godel_numbers[j]))))
        .collect();
    distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
    for (i, j, distance) in distances.iter().take(5) {
        println!("  {} ↔ {}: {:.4}", godel_numbers[*i].entity, godel_numbers[*j].entity, distance);
    }
    println!();

    let numbers: Vec<u64> = godel_numbers.iter().map(|g| g.value).collect();
    let center = GodelNumber::<Stage>::geometric_center(&numbers);
    let variance = GodelNumber::<Stage>::variance(&numbers);
    let avg_distance = GodelNumber::<Stage>::average_pairwise_distance(&numbers);

    println!("📐 Geometric Analysis:");
    println!("  Geometric Center: {:.2}", center);
    println!("  Variance: {:.2}", variance);
    println!("  Average Pairwise Distance: {:.4}", avg_distance);
    println!();

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

    println!("🧬 8D Prime Exponent Vectors:");
    for godel in &godel_numbers {
        println!("  {} {:?}: {:?}", godel.entity, godel.entity, godel.to_8d_vector());
    }
    println!();

    println!("📏 8D Euclidean Distances:");
    for i in 0..godel_numbers.len() {
        for j in (i + 1)..godel_numbers.len() {
            let distance = godel_numbers[i].euclidean_distance_8d(&godel_numbers[j]);
            println!("  {} ↔ {}: {:.4}", godel_numbers[i].entity, godel_numbers[j].entity, distance);
        }
    }
    println!();

    println!("🎵 Resonance Analysis:");
    for godel in &godel_numbers[1..] {
        let distance = godel.distance_to(&godel_numbers[0]);
        let resonance = 1.0 / (1.0 + distance);
        println!("  {} ↔ {}: resonance {:.4}", godel_numbers[0].entity, godel.entity, resonance);
    }
    println!();

    println!("✨ Euclidean distance analysis complete!");
}

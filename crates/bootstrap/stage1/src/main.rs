//! Stage 1 Main: Demonstrating Duality

use bootstrap_stage0::{
    ChecksumHasher, DefaultProjector, DefaultProofVerifier, Describable, Kernel, Proof,
};
use bootstrap_stage1::{dualize, Dualizer};

fn main() {
    println!("--- Stage 1: Duality ---");

    // 1. Initialize the Kernel
    let mut kernel = Kernel::new(
        Box::new(DefaultProofVerifier::new()),
        Box::new(DefaultProjector),
        Box::new(ChecksumHasher),
    );
    let hasher = ChecksumHasher;

    // 2. Define the components of our system
    let genesis_content = b"The first vibe.".to_vec();
    let dualizer = Dualizer;

    // 3. Create proofs for the atomic components
    let genesis_hash = hasher.hash(&genesis_content);
    let genesis_proof = Proof::Atomic(genesis_hash.clone());

    let dualizer_proof = dualizer.prove();
    let dualizer_hash = dualizer_proof.hash().clone();

    // 4. Ingest the atomic proofs into the Kernel's lattice
    kernel.ingest(genesis_proof).expect("Failed to ingest Genesis proof");
    println!("✅ Ingested Genesis Proof: {:?}", genesis_hash);

    kernel.ingest(dualizer_proof).expect("Failed to ingest Dualizer proof");
    println!("✅ Ingested Dualizer Proof: {:?}", dualizer_hash);

    // 5. Execute the duality reflection
    let (_dual_content, dual_hash) = dualize(&genesis_content, &hasher);
    let dual_proof = Proof::Atomic(dual_hash.clone());

    kernel.ingest(dual_proof).expect("Failed to ingest Dual proof");
    println!("✅ Ingested Dual Proof: {:?}", dual_hash);

    // 6. Create and ingest the Composite Proof that harmonizes the system
    let composite_hash = hasher.hash(
        &[genesis_hash.hash_bytes, dualizer_hash.hash_bytes].concat(),
    );
    let composite_proof = Proof::Composite {
        hash: composite_hash,
        components: vec![genesis_hash, dualizer_hash],
    };

    kernel.ingest(composite_proof).expect("Failed to ingest Composite proof");
    println!("✅ Ingested Composite Proof, harmonizing Genesis and Dualizer.");

    println!("\n--- System State ---");
    println!("Kernel Step: {}", kernel.step);
    println!("Coordinates on Manifold:");
    println!(
        "  Genesis:  {:?}",
        kernel.get_coordinate(&genesis_hash).unwrap()
    );
    println!(
        "  Dualizer: {:?}",
        kernel.get_coordinate(&dualizer_hash).unwrap()
    );
    println!("  Dual:     {:?}", kernel.get_coordinate(&dual_hash).unwrap());
}

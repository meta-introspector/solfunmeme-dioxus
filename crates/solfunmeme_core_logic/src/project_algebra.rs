#[derive(Debug, Clone, Copy)]
pub enum Feature {
    Developer,      // 🧑‍💻 2
    Package,        // 📦 3
    Introspection,  // 🔍 5
    Fun,            // ✨ 7
    RustLang,       // 🦀 11
    Web,            // 🌐 13
    Modular,        // 🧩 17
    Docs,           // 📝 19
}

impl Feature {
    pub fn prime(&self) -> u64 {
        match self {
            Feature::Developer     => 2,
            Feature::Package       => 3,
            Feature::Introspection => 5,
            Feature::Fun           => 7,
            Feature::RustLang      => 11,
            Feature::Web           => 13,
            Feature::Modular       => 17,
            Feature::Docs          => 19,
        }
    }
}

#[derive(Debug)]
pub struct Project {
    pub features: Vec<Feature>,
}

impl Project {
    pub fn new(features: Vec<Feature>) -> Self {
        Project { features }
    }

    // Product: unique project signature
    pub fn signature(&self) -> u64 {
        self.features.iter().map(|f| f.prime()).product()
    }

    // Sum: fun/introspection layer
    pub fn sum(&self) -> u64 {
        self.features.iter().map(|f| f.prime()).sum()
    }
}

fn main() {
    // Core project structure
    let core_project = Project::new(vec![
        Feature::Developer,
        Feature::Package,
        Feature::Modular,
        Feature::RustLang,
        Feature::Web,
    ]);
    println!("Core Project Signature (Product): {}", core_project.signature()); // 4862

    // Fun/introspection layer
    let fun_layer = Project::new(vec![
        Feature::Fun,
        Feature::Introspection,
        Feature::Docs,
    ]);
    println!("Fun Layer (Sum): {}", fun_layer.sum()); // 31

    // Full project signature
    let full_project = Project::new(vec![
        Feature::Developer,
        Feature::Package,
        Feature::Introspection,
        Feature::Fun,
        Feature::RustLang,
        Feature::Web,
        Feature::Modular,
        Feature::Docs,
    ]);
    println!("Full Project Signature (Product): {}", full_project.signature()); // 9699690

    // Web components in Rust
    let web_components = Project::new(vec![
        Feature::Web,
        Feature::Modular,
        Feature::RustLang,
    ]);
    println!("Web Components Signature (Product): {}", web_components.signature()); // 2431
}

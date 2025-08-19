use std::env;

#[derive(Debug, PartialEq)]
pub enum TestCategory {
    All,
    Vectorization,
    DeclarationSplitting,
    DuplicateDetection,
    CodeAnalysis,
    WalletIntegration,
    SelfAnalysis,
}

impl TestCategory {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "all" => Some(TestCategory::All),
            "vectorization" => Some(TestCategory::Vectorization),
            "declarationsplitting" => Some(TestCategory::DeclarationSplitting),
            "duplicatedetection" => Some(TestCategory::DuplicateDetection),
            "codeanalysis" => Some(TestCategory::CodeAnalysis),
            "walletintegration" => Some(TestCategory::WalletIntegration),
            "selfanalysis" => Some(TestCategory::SelfAnalysis),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct TestSuiteArgs {
    pub category: TestCategory,
}

impl TestSuiteArgs {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let mut args = env::args().skip(1);
        let mut category = TestCategory::All;
        
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--run" => {
                    if let Some(cat_str) = args.next() {
                        if let Some(cat) = TestCategory::from_str(&cat_str) {
                            category = cat;
                        } else {
                            eprintln!("Unknown test category: {}. Running all tests.", cat_str);
                        }
                    }
                },
                "--help" | "-h" => {
                    print_help();
                    std::process::exit(0);
                },
                _ => {
                    eprintln!("Unknown argument: {}. Running all tests.", arg);
                }
            }
        }

        Ok(Self {
            category,
        })
    }
}

fn print_help() {
    println!("Usage: test_runner [OPTIONS]");
    println!();
    println!("Options:");
    println!("  --run CATEGORY      Run specific test category (e.g., vectorization, all)");
    println!("  --help, -h          Show this help message");
    println!();
    println!("Available Categories:");
    println!("  all");
    println!("  vectorization");
    println!("  declarationsplitting");
    println!("  duplicatedetection");
    println!("  codeanalysis");
    println!("  walletintegration");
    println!("  selfanalysis");
    println!();
    println!("Examples:");
    println!("  test_runner --run all                     # Run all tests");
    println!("  test_runner --run vectorization           # Run only vectorization tests");
}

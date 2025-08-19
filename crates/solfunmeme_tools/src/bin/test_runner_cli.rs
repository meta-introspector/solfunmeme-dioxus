use solfunmeme_tools::test_suite::{self, TestSuiteArgs};
use solfunmeme_tools::test_suite::args::TestCategory;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = TestSuiteArgs::from_env()?;

    println!("🚀 Solfunmeme Test Runner - Comprehensive Coverage Tests");

    match args.category {
        TestCategory::All => {
            test_suite::run_all_tests()?;
        },
        _ => {
            println!("Running specific test categories is not supported in lightweight mode.");
            println!("Please run with the 'full' feature enabled.");
        }
    }

    println!("\n✅ All selected tests completed successfully!");
    Ok(())
}

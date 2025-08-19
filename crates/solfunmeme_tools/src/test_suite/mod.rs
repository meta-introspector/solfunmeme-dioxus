pub mod args;
#[cfg(feature = "full")]
pub mod vectorization_tests;
#[cfg(feature = "full")]
pub mod declaration_splitting_tests;
#[cfg(feature = "full")]
pub mod duplicate_detection_tests;
#[cfg(feature = "full")]
pub mod code_analysis_tests;
#[cfg(feature = "full")]
pub mod self_analysis_tests;
#[cfg(feature = "full")]
pub mod wallet_integration_tests;

pub use args::TestSuiteArgs;
#[cfg(feature = "full")]
pub use vectorization_tests::*;
#[cfg(feature = "full")]
pub use declaration_splitting_tests::*;
#[cfg(feature = "full")]
pub use duplicate_detection_tests::*;
#[cfg(feature = "full")]
pub use code_analysis_tests::*;
#[cfg(feature = "full")]
pub use self_analysis_tests::*;
#[cfg(feature = "full")]
pub use wallet_integration_tests::*;

#[cfg(not(feature = "full"))]
pub fn run_all_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("Test suite requires 'full' feature to be enabled");
    Ok(())
}

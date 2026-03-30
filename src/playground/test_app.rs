// # Use Cases Identified from Code Coverage Report

// ## Core Wallet Functions
// 1. **Cluster Management**
//    - Add/remove Solana clusters
//    - Set active cluster
//    - Validate cluster endpoints
//    - Manage cluster configurations

// 2. **Cryptographic Operations**
//    - Generate keypairs
//    - Sign/verify messages
//    - Encrypt/decrypt data
//    - Validate public/private keys
//    - Key derivation and management

// 3. **Account Management**
//    - Query account states
//    - Manage token accounts
//    - Account balance tracking
//    - Account creation and validation

// 4. **Transaction Operations**
//    - Send SOL transfers
//    - Sign transactions
//    - Transaction history
//    - Fee calculation

// 5. **Meme/NFT Features**
//    - Component memes management
//    - Workflow memes processing
//    - Wikidata memes integration
//    - Meme metadata handling

// 6. **Lean Theorem Prover Integration**
//    - Expression parsing and validation
//    - Emoji representation of expressions
//    - Level management
//    - Binder operations

// 7. **Notification System**
//    - Error notifications
//    - Success messages
//    - Timed notifications
//    - User feedback

// 8. **Connection Management**
//    - Wallet adapter connections
//    - Connection filtering
//    - Connection state management
//    - Multi-wallet support

// ---

// # Dioxus Test Menu Structure

// ```rust
use dioxus::prelude::*;

// NEW CODE

//use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum TestCategory {
    CryptoOperations,
    WalletOperations,
    ClusterManagement,
    AccountOperations,
    TransactionOps,
    MemeFeatures,
    LeanIntegration,
    NotificationSystem,
    ConnectionManagement,
    UtilityFunctions,
    TimerFunctions,
}

#[derive(Clone, PartialEq)]
pub struct TestCategory2 {
    pub name: String,
    pub description: String,
    pub category: TestCategory,
    pub test_fn: fn() -> Result<String, String>,
}

#[derive(Clone, PartialEq)]
pub struct TestCase {
    pub name: String,
    pub description: String,
    pub category: TestCategory,
    pub test_fn: fn() -> Result<String, String>,
}

#[component]
pub fn TestMenuApp() -> Element {
    let mut selected_category = use_signal(|| TestCategory::CryptoOperations);
    let mut test_results = use_signal(|| Vec::<(String, Result<String, String>)>::new());
    let mut show_results = use_signal(|| false);
    let test_cases = use_signal(|| get_test_cases());

    // Use use_memo to cache filtered test cases
    let filtered_cases = use_memo(move || {
        test_cases
            .read()
            .iter()
            .filter(|tc| tc.category == selected_category())
            .cloned()
            .collect::<Vec<TestCase>>()
    });

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6 text-center",
                "SolFunMeme Test Suite"
            }

            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Category Sidebar
                div { class: "lg:col-span-1",
                    div { class: "bg-white shadow-lg rounded-lg p-4",
                        h2 { class: "text-xl font-semibold mb-4", "Test Categories" }
                        for category in [
                            TestCategory::CryptoOperations,
                            TestCategory::ClusterManagement,
                            TestCategory::AccountOperations,
                            TestCategory::TransactionOps,
                            TestCategory::MemeFeatures,
                            TestCategory::LeanIntegration,
                            TestCategory::NotificationSystem,
                            TestCategory::ConnectionManagement,
                            TestCategory::UtilityFunctions,
                        ] {
                            button {
                                class: format!(
                                    "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
                                    if selected_category() == category {
                                        "bg-blue-500 text-white"
                                    } else {
                                        "bg-gray-100 hover:bg-gray-200"
                                    }
                                ),
                                onclick: move |_| selected_category.set(category.clone()),
                                "{category_name(&category)}"
                            }
                        }
                    }
                }

                // Test Cases Panel
                div { class: "lg:col-span-3",
                    div { class: "bg-white shadow-lg rounded-lg p-6",
                        div { class: "flex justify-between items-center mb-4",
                            h2 { class: "text-2xl font-semibold",
                                "{category_name(&selected_category())}"
                            }
                            button {
                                class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
                                onclick: move |_| run_category_tests(selected_category(), &mut test_results, &mut show_results),
                                "Run All Tests"
                            }
                        }

                        div { class: "grid gap-4",
                            for test_case in filtered_cases() {
                                TestCaseCard {
                                    test_case: test_case.clone(),
                                    on_run: move |result| {
                                        let mut results = test_results();
                                        results.push((test_case.name.clone(), result));
                                        test_results.set(results);
                                        show_results.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Results Panel
            if show_results() {
                div { class: "mt-6 bg-white shadow-lg rounded-lg p-6",
                    div { class: "flex justify-between items-center mb-4",
                        h2 { class: "text-2xl font-semibold", "Test Results" }
                        button {
                            class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
                            onclick: move |_| {
                                test_results.set(Vec::new());
                                show_results.set(false);
                            },
                            "Clear Results"
                        }
                    }
                    div { class: "max-h-96 overflow-y-auto",
                        for (test_name, result) in test_results().iter() {
                            div { class: format!(
                                "p-3 mb-2 rounded-lg {}",
                                match result {
                                    Ok(_) => "bg-green-100 border-green-300",
                                    Err(_) => "bg-red-100 border-red-300",
                                }
                            ),
                                div { class: "font-semibold", "{test_name}" }
                                div { class: "text-sm",
                                    match result {
                                        Ok(msg) => rsx! { span { class: "text-green-700", "✓ {msg}" } },
                                        Err(msg) => rsx! { span { class: "text-red-700", "✗ {msg}" } },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// #[component]
// pub fn TestMenuApp3() -> Element {
//     let mut selected_category = use_signal(|| TestCategory::CryptoOperations);
//     let mut test_results = use_signal(|| Vec::<(String, Result<String, String>)>::new());
//     let mut show_results = use_signal(|| false);
//     let mut test_cases = use_signal(|| get_test_cases()); // Store test_cases in use_ref

//     rsx! {
//         div { class: "container mx-auto p-4",
//             h1 { class: "text-3xl font-bold mb-6 text-center",
//                 "SolFunMeme Test Suite"
//             }

//             div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
//                 // Category Sidebar
//                 div { class: "lg:col-span-1",
//                     div { class: "bg-white shadow-lg rounded-lg p-4",
//                         h2 { class: "text-xl font-semibold mb-4", "Test Categories" }
//                         for category in [
//                             TestCategory::CryptoOperations,
//                             TestCategory::ClusterManagement,
//                             TestCategory::AccountOperations,
//                             TestCategory::TransactionOps,
//                             TestCategory::MemeFeatures,
//                             TestCategory::LeanIntegration,
//                             TestCategory::NotificationSystem,
//                             TestCategory::ConnectionManagement,
//                             TestCategory::UtilityFunctions,
//                         ] {
//                             button {
//                                 class: format!(
//                                     "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
//                                     if selected_category() == category {
//                                         "bg-blue-500 text-white"
//                                     } else {
//                                         "bg-gray-100 hover:bg-gray-200"
//                                     }
//                                 ),
//                                 onclick: move |_| selected_category.set(category.clone()),
//                                 "{category_name(&category)}"
//                             }
//                         }
//                     }
//                 }

//                 // Test Cases Panel
//                 div { class: "lg:col-span-3",
//                     div { class: "bg-white shadow-lg rounded-lg p-6",
//                         div { class: "flex justify-between items-center mb-4",
//                             h2 { class: "text-2xl font-semibold",
//                                 "{category_name(&selected_category())}"
//                             }
//                             button {
//                                 class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
//                                 onclick: move |_| run_category_tests(selected_category(), &mut test_results, &mut show_results),
//                                 "Run All Tests"
//                             }
//                         }

//                         div { class: "grid gap-4",
//                             for test_case in test_cases.read().iter().filter(|tc| tc.category == selected_category()) {
//                                 TestCaseCard {
//                                     test_case: test_case.clone(),
//                                     on_run: move |result| {
//                                         let mut results = test_results();
//                                         results.push((test_case.name.clone(), result));
//                                         test_results.set(results);
//                                         show_results.set(true);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             // Results Panel
//             if show_results() {
//                 div { class: "mt-6 bg-white shadow-lg rounded-lg p-6",
//                     div { class: "flex justify-between items-center mb-4",
//                         h2 { class: "text-2xl font-semibold", "Test Results" }
//                         button {
//                             class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
//                             onclick: move |_| {
//                                 test_results.set(Vec::new());
//                                 show_results.set(false);
//                             },
//                             "Clear Results"
//                         }
//                     }
//                     div { class: "max-h-96 overflow-y-auto",
//                         for (test_name, result) in test_results().iter() {
//                             div { class: format!(
//                                 "p-3 mb-2 rounded-lg {}",
//                                 match result {
//                                     Ok(_) => "bg-green-100 border-green-300",
//                                     Err(_) => "bg-red-100 border-red-300",
//                                 }
//                             ),
//                                 div { class: "font-semibold", "{test_name}" }
//                                 div { class: "text-sm",
//                                     match result {
//                                         Ok(msg) => rsx! { span { class: "text-green-700", "✓ {msg}" } },
//                                         Err(msg) => rsx! { span { class: "text-red-700", "✗ {msg}" } },
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

// #[component]
// pub fn TestMenuApp21() -> Element {
//     let mut selected_category = use_signal(|| TestCategory::CryptoOperations);
//     let mut test_results = use_signal(|| Vec::<(String, Result<String, String>)>::new());
//     let mut show_results = use_signal(|| false);

//     let test_cases = get_test_cases();

//     rsx! {
//         div { class: "container mx-auto p-4",
//             h1 { class: "text-3xl font-bold mb-6 text-center",
//                 "SolFunMeme Test Suite"
//             }

//             div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
//                 // Category Sidebar
//                 div { class: "lg:col-span-1",
//                     div { class: "bg-white shadow-lg rounded-lg p-4",
//                         h2 { class: "text-xl font-semibold mb-4", "Test Categories" }
//                         for category in [
//                             TestCategory::CryptoOperations,
//                             TestCategory::ClusterManagement,
//                             TestCategory::AccountOperations,
//                             TestCategory::TransactionOps,
//                             TestCategory::MemeFeatures,
//                             TestCategory::LeanIntegration,
//                             TestCategory::NotificationSystem,
//                             TestCategory::ConnectionManagement,
//                             TestCategory::UtilityFunctions,
//                         ] {
//                             button {
//                                 class: format!(
//                                     "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
//                                     if selected_category() == category {
//                                         "bg-blue-500 text-white"
//                                     } else {
//                                         "bg-gray-100 hover:bg-gray-200"
//                                     }
//                                 ),
//                                 onclick: move |_| selected_category.set(category.clone()),
//                                 "{category_name(&category)}"
//                             }
//                         }
//                     }
//                 }

//                 // Test Cases Panel
//                 div { class: "lg:col-span-3",
//                     div { class: "bg-white shadow-lg rounded-lg p-6",
//                         div { class: "flex justify-between items-center mb-4",
//                             h2 { class: "text-2xl font-semibold",
//                                 "{category_name(&selected_category())}"
//                             }
//                             button {
//                                 class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
//                                 onclick: move |_| run_category_tests(selected_category(), &mut test_results, &mut show_results),
//                                 "Run All Tests"
//                             }
//                         }

//                         div { class: "grid gap-4",
//                             for test_case in test_cases.iter().filter(|tc| tc.category == selected_category()) {
//                                 TestCaseCard {
//                                     test_case: test_case.clone(),
//                                     on_run: move |result| {
//                                         let mut results = test_results();
//                                         results.push((test_case.name.clone(), result));
//                                         test_results.set(results);
//                                         show_results.set(true);
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }

//             // Results Panel
//             if show_results() {
//                 div { class: "mt-6 bg-white shadow-lg rounded-lg p-6",
//                     div { class: "flex justify-between items-center mb-4",
//                         h2 { class: "text-2xl font-semibold", "Test Results" }
//                         button {
//                             class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
//                             onclick: move |_| {
//                                 test_results.set(Vec::new());
//                                 show_results.set(false);
//                             },
//                             "Clear Results"
//                         }
//                     }
//                     div { class: "max-h-96 overflow-y-auto",
//                         for (test_name, result) in test_results().iter() {
//                             div { class: format!(
//                                 "p-3 mb-2 rounded-lg {}",
//                                 match result {
//                                     Ok(_) => "bg-green-100 border-green-300",
//                                     Err(_) => "bg-red-100 border-red-300",
//                                 }
//                             ),
//                                 div { class: "font-semibold", "{test_name}" }
//                                 div { class: "text-sm",
//                                     match result {
//                                         Ok(msg) => rsx! { span { class: "text-green-700", "✓ {msg}" } },
//                                         Err(msg) => rsx! { span { class: "text-red-700", "✗ {msg}" } },
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

#[component]
pub fn TestMenuApp2() -> Element {
    let mut selected_category = use_signal(|| TestCategory::CryptoOperations);
    let mut test_results = use_signal(|| Vec::<(String, Result<String, String>)>::new());
    let mut show_results = use_signal(|| false);
    let test_cases = get_test_cases();

    // Collect filtered test cases into a Vec
    let filtered_cases: Vec<TestCase> = test_cases
        .iter()
        .filter(|tc| tc.category == selected_category())
        .cloned()
        .collect();

    rsx! {
        div { class: "container mx-auto p-4",
            h1 { class: "text-3xl font-bold mb-6 text-center",
                "SolFunMeme Test Suite"
            }

            div { class: "grid grid-cols-1 lg:grid-cols-4 gap-6",
                // Category Sidebar
                div { class: "lg:col-span-1",
                    div { class: "bg-white shadow-lg rounded-lg p-4",
                        h2 { class: "text-xl font-semibold mb-4", "Test Categories" }
                        for category in [
                            TestCategory::CryptoOperations,
                            TestCategory::ClusterManagement,
                            TestCategory::AccountOperations,
                            TestCategory::TransactionOps,
                            TestCategory::MemeFeatures,
                            TestCategory::LeanIntegration,
                            TestCategory::NotificationSystem,
                            TestCategory::ConnectionManagement,
                            TestCategory::UtilityFunctions,
                        ] {
                            button {
                                class: format!(
                                    "w-full text-left p-3 mb-2 rounded-lg transition-colors {}",
                                    if selected_category() == category {
                                        "bg-blue-500 text-white"
                                    } else {
                                        "bg-gray-100 hover:bg-gray-200"
                                    }
                                ),
                                onclick: move |_| selected_category.set(category.clone()),
                                "{category_name(&category)}"
                            }
                        }
                    }
                }

                // Test Cases Panel
                div { class: "lg:col-span-3",
                    div { class: "bg-white shadow-lg rounded-lg p-6",
                        div { class: "flex justify-between items-center mb-4",
                            h2 { class: "text-2xl font-semibold",
                                "{category_name(&selected_category())}"
                            }
                            button {
                                class: "bg-green-500 text-white px-4 py-2 rounded-lg hover:bg-green-600",
                                onclick: move |_| run_category_tests(selected_category(), &mut test_results, &mut show_results),
                                "Run All Tests"
                            }
                        }

                        div { class: "grid gap-4",
                            for test_case in filtered_cases {
                                TestCaseCard {
                                    test_case: test_case.clone(),
                                    on_run: move |result| {
                                        let mut results = test_results();
                                        results.push((test_case.name.clone(), result));
                                        test_results.set(results);
                                        show_results.set(true);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Results Panel
            if show_results() {
                div { class: "mt-6 bg-white shadow-lg rounded-lg p-6",
                    div { class: "flex justify-between items-center mb-4",
                        h2 { class: "text-2xl font-semibold", "Test Results" }
                        button {
                            class: "bg-red-500 text-white px-4 py-2 rounded-lg hover:bg-red-600",
                            onclick: move |_| {
                                test_results.set(Vec::new());
                                show_results.set(false);
                            },
                            "Clear Results"
                        }
                    }
                    div { class: "max-h-96 overflow-y-auto",
                        for (test_name, result) in test_results().iter() {
                            div { class: format!(
                                "p-3 mb-2 rounded-lg {}",
                                match result {
                                    Ok(_) => "bg-green-100 border-green-300",
                                    Err(_) => "bg-red-100 border-red-300",
                                }
                            ),
                                div { class: "font-semibold", "{test_name}" }
                                div { class: "text-sm",
                                    match result {
                                        Ok(msg) => rsx! { span { class: "text-green-700", "✓ {msg}" } },
                                        Err(msg) => rsx! { span { class: "text-red-700", "✗ {msg}" } },
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TestCaseCard(test_case: TestCase, on_run: EventHandler<Result<String, String>>) -> Element {
    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow",
            div { class: "flex justify-between items-start mb-2",
                h3 { class: "font-medium text-lg", "{test_case.name}" }
                button {
                    class: "bg-blue-500 text-white px-3 py-1 rounded text-sm hover:bg-blue-600",
                    onclick: move |_| {
                        let result = (test_case.test_fn)();
                        on_run.call(result);
                    },
                    "Run Test"
                }
            }
            p { class: "text-gray-600 text-sm", "{test_case.description}" }
        }
    }
}

fn category_name(category: &TestCategory) -> &'static str {
    match category {
        TestCategory::CryptoOperations => "Crypto Operations",
        TestCategory::TimerFunctions => "Timer Functions",
        TestCategory::WalletOperations => "Wallet Operations",
        TestCategory::ClusterManagement => "Cluster Management",
        TestCategory::AccountOperations => "Account Operations",
        TestCategory::TransactionOps => "Transaction Operations",
        TestCategory::MemeFeatures => "Meme Features",
        TestCategory::LeanIntegration => "Lean Integration",
        TestCategory::NotificationSystem => "Notification System",
        TestCategory::ConnectionManagement => "Connection Management",
        TestCategory::UtilityFunctions => "Utility Functions",
    }
}

fn run_category_tests(
    category: TestCategory,
    test_results: &mut Signal<Vec<(String, Result<String, String>)>>,
    show_results: &mut Signal<bool>,
) {
    let test_cases = get_test_cases();
    let mut results = Vec::new();

    for test_case in test_cases.iter().filter(|tc| tc.category == category) {
        let result = (test_case.test_fn)();
        results.push((test_case.name.clone(), result));
    }

    test_results.set(results);
    show_results.set(true);
}

// Test function implementations (placeholder)
fn test_keypair_generation() -> Result<String, String> {
    Ok("Keypair generation successful".to_string())
}

fn test_message_signing() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_data_encryption() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_key_validation() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_add_cluster() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_set_active_cluster() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_cluster_validation() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_account_query() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_token_account() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_balance() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_sol_transfer() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_transaction_signing() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_fee_calculation() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_workflow_memes() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_wikidata_integration() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_expression() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_emoji() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_level_management() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_notification_creation() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_timed_notification() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_wallet_connection() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_connection_filtering() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_data_parsing() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_storage_operation() -> Result<String, String> {
    Ok("Test successful".to_string())
}

// OLD CODE
#[derive(Clone, PartialEq)]
pub enum TestCategory22 {
    CryptoOperations,
    ClusterManagement,
    AccountOperations,
    TransactionOps,
    MemeFeatures,
    LeanIntegration,
    NotificationSystem,
    ConnectionManagement,
    UtilityFunctions,
}

#[derive(Clone, PartialEq)]
pub struct TestCase2 {
    pub name: String,
    pub description: String,
    pub category: TestCategory,
    pub test_fn: fn() -> Result<String, String>,
}

//use dioxus::prelude::*;

#[component]
fn TestCaseCard2(test_case: TestCase, on_run: EventHandler<Result<String, String>>) -> Element {
    rsx! {
        div { class: "border border-gray-200 rounded-lg p-4 hover:shadow-md transition-shadow",
            div { class: "flex justify-between items-start mb-2",
                h3 { class: "font-medium text-lg", "{test_case.name}" }
                button {
                    class: "bg-blue-500 text-white px-3 py-1 rounded text-sm hover:bg-blue-600",
                    onclick: move |_| {
                        let result = (test_case.test_fn)();
                        on_run.call(result);
                    },
                    "Run Test"
                }
            }
            p { class: "text-gray-600 text-sm", "{test_case.description}" }
        }
    }
}

// fn category_name(category: &TestCategory) -> &'static str {
//     match category {
//         TestCategory::CryptoOperations => "Crypto Operations",
//         TestCategory::ClusterManagement => "Cluster Management",
//         TestCategory::AccountOperations => "Account Operations",
//         TestCategory::TransactionOps => "Transaction Operations",
//         TestCategory::MemeFeatures => "Meme Features",
//         TestCategory::LeanIntegration => "Lean Integration",
//         TestCategory::NotificationSystem => "Notification System",
//         TestCategory::ConnectionManagement => "Connection Management",
//         TestCategory::UtilityFunctions => "Utility Functions",
//     }
// }

// fn run_category_tests(
//     category: TestCategory,
//     test_results: &mut Signal<Vec<(String, Result<String, String>)>>,
//     show_results: &mut Signal<bool>
// ) {
//     let test_cases = get_test_cases();
//     let mut results = Vec::new();

//     for test_case in test_cases.iter().filter(|tc| tc.category == category) {
//         let result = (test_case.test_fn)();
//         results.push((test_case.name.clone(), result));
//     }

//     test_results.set(results);
//     show_results.set(true);
// }

// Test function implementations (placeholder - implement based on your actual test logic)
fn test_keypair_generation2() -> Result<String, String> {
    // Call your actual crypto::test_keypair_generation
    Ok("Keypair generation successful".to_string())
}

fn test_message_signing2() -> Result<String, String> {
    // Call your actual crypto signing tests
    Ok("Message signing verified".to_string())
}

fn test_data_encryption2() -> Result<String, String> {
    // Call your actual encryption tests
    Ok("Encryption/decryption cycle completed".to_string())
}

fn test_key_validation2() -> Result<String, String> {
    // Call your actual key validation tests
    Ok("Key validation passed".to_string())
}

fn test_add_cluster2() -> Result<String, String> {
    // Call your actual cluster management tests
    Ok("Cluster added successfully".to_string())
}

fn test_set_active_cluster2() -> Result<String, String> {
    Ok("Active cluster set".to_string())
}

fn test_cluster_validation2() -> Result<String, String> {
    Ok("Cluster validation passed".to_string())
}

fn test_account_query2() -> Result<String, String> {
    Ok("Account query successful".to_string())
}

fn test_token_account_management() -> Result<String, String> {
    Ok("Token account management verified".to_string())
}

fn test_balance_tracking() -> Result<String, String> {
    Ok("Balance tracking functional".to_string())
}

fn test_component_meme() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_expression_parsing() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn expression_parsing() -> Result<String, String> {
    Ok("Test successful".to_string())
}

fn test_emoji_conversion() -> Result<String, String> {
    Ok("Test successful".to_string())
}

//test_emoji_conversion

// This menu structure provides:

// 1. **Categorized test organization** - Tests are grouped by functionality
// 2. **Interactive UI** - Click-to-run individual tests or entire categories
// 3. **Real-time results** - See test outcomes immediately
// 4. **Visual feedback** - Color-coded success/failure indicators
// 5. **Comprehensive coverage** - Addresses all major components from your coverage report
// 6. **Extensible design** - Easy to add new test cases and categories

// The placeholder test functions should be replaced with actual calls to your existing test modules to provide real functionality testing.

fn get_test_cases() -> Vec<TestCase> {
    vec![
        // Crypto Operations
        TestCase {
            name: "Keypair Generation".to_string(),
            description: "Test generating new cryptographic keypairs".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_keypair_generation,
        },
        TestCase {
            name: "Message Signing".to_string(),
            description: "Test signing and verifying messages".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_message_signing,
        },
        TestCase {
            name: "Data Encryption".to_string(),
            description: "Test encrypting and decrypting data".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_data_encryption,
        },
        TestCase {
            name: "Key Validation".to_string(),
            description: "Test validation of public and private keys".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_key_validation,
        },
        // Cluster Management
        //TestCategory2 {
        TestCase {
            name: "Add Cluster".to_string(),
            description: "Test adding new Solana clusters".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_add_cluster,
        },
        TestCase {
            name: "Set Active Cluster".to_string(),
            description: "Test setting the active cluster".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_set_active_cluster,
        },
        TestCase {
            name: "Cluster Validation".to_string(),
            description: "Test cluster endpoint validation".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_cluster_validation,
        },
        // Account Operations
        TestCase {
            name: "Account Query".to_string(),
            description: "Test querying account information".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_account_query,
        },
        TestCase {
            name: "Token Account Management".to_string(),
            description: "Test managing token accounts".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_token_account_management,
        },
        TestCase {
            name: "Balance Tracking".to_string(),
            description: "Test account balance tracking".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_balance_tracking,
        },
        // Transaction Operations
        TestCase {
            name: "SOL Transfer".to_string(),
            description: "Test sending SOL transfers".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_transaction_signing,
        },
        TestCase {
            name: "Transaction Signing".to_string(),
            description: "Test transaction signing process".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_transaction_signing,
        },
        TestCase {
            name: "Fee Calculation".to_string(),
            description: "Test transaction fee calculation".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_fee_calculation,
        },
        // Meme Features
        TestCase {
            name: "Component Memes".to_string(),
            description: "Test component meme functionality".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_component_meme,
        },
        TestCase {
            name: "Workflow Memes".to_string(),
            description: "Test workflow meme processing".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_workflow_memes,
        },
        TestCase {
            name: "Wikidata Integration".to_string(),
            description: "Test Wikidata meme integration".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_wikidata_integration,
        },
        // Lean Integration
        TestCase {
            name: "Expression Parsing".to_string(),
            description: "Test parsing Lean expressions".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: expression_parsing,
        },
        TestCase {
            name: "Emoji Conversion".to_string(),
            description: "Test converting expressions to emoji representation".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: test_emoji_conversion,
        },
        TestCase {
            name: "Level Management".to_string(),
            description: "Test Lean level operations".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: test_level_management,
        },
        // Notification System
        TestCase {
            name: "SystemSystemNotification Creation".to_string(),
            description: "Test creating notifications".to_string(),
            category: TestCategory::NotificationSystem,
            test_fn: test_notification_creation,
        },
        TestCase {
            name: "SystemTimed Notifications".to_string(),
            description: "Test timed notifications functionality".to_string(),
            category: TestCategory::TimerFunctions,
            test_fn: test_timed_notification,
        },
        TestCase {
            name: "Wallet Connection".to_string(),
            description: "testConnectionDescription".to_string(),
            category: TestCategory::WalletOperations,
            test_fn: test_wallet_connection,
        },
        TestCase {
            name: "Connection Filtering".to_string(),
            description: "Filtering functionality".to_string(),
            category: TestCategory::ConnectionManagement,
            test_fn: test_connection_filtering,
        },
        TestCase {
            name: "Data Parsing".to_string(),
            description: "Testing data parsing".to_string(),
            category: TestCategory::UtilityFunctions,
            test_fn: test_data_parsing,
        },
        TestCase {
            name: "Storage Operations".to_string(),
            description: "Test storage utility".to_string(),
            category: TestCategory::UtilityFunctions,
            test_fn: test_storage_operation,
        },
    ]
}
fn get_test_cases2() -> Vec<TestCase> {
    vec![
        // Crypto Operations
        TestCase {
            name: "Keypair Generation".to_string(),
            description: "Test generating new cryptographic keypairs".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_keypair_generation,
        },
        TestCase {
            name: "Message Signing".to_string(),
            description: "Test signing and verifying messages".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_message_signing,
        },
        TestCase {
            name: "Data Encryption".to_string(),
            description: "Test encrypting and decrypting data".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_data_encryption,
        },
        TestCase {
            name: "Key Validation".to_string(),
            description: "Test validation of public and private keys".to_string(),
            category: TestCategory::CryptoOperations,
            test_fn: test_key_validation,
        },
        // Cluster Management
        TestCase {
            name: "Add Cluster".to_string(),
            description: "Test adding new Solana clusters".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_add_cluster,
        },
        TestCase {
            name: "Set Active Cluster".to_string(),
            description: "Test setting the active cluster".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_set_active_cluster,
        },
        TestCase {
            name: "Cluster Validation".to_string(),
            description: "Test cluster endpoint validation".to_string(),
            category: TestCategory::ClusterManagement,
            test_fn: test_cluster_validation,
        },
        // Account Operations
        TestCase {
            name: "Account Query".to_string(),
            description: "Test querying account information".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_account_query,
        },
        TestCase {
            name: "Token Account Management".to_string(),
            description: "Test managing token accounts".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_token_account_management,
        },
        TestCase {
            name: "Balance Tracking".to_string(),
            description: "Test account balance tracking".to_string(),
            category: TestCategory::AccountOperations,
            test_fn: test_balance_tracking,
        },
        // Transaction Operations
        TestCase {
            name: "SOL Transfer".to_string(),
            description: "Test sending SOL transfers".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_sol_transfer,
        },
        TestCase {
            name: "Transaction Signing".to_string(),
            description: "Test transaction signing process".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_transaction_signing,
        },
        TestCase {
            name: "Fee Calculation".to_string(),
            description: "Test transaction fee calculation".to_string(),
            category: TestCategory::TransactionOps,
            test_fn: test_fee_calculation,
        },
        // Meme Features
        TestCase {
            name: "Component Memes".to_string(),
            description: "Test component meme functionality".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_component_meme,
        },
        TestCase {
            name: "Workflow Memes".to_string(),
            description: "Test workflow meme processing".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_workflow_memes,
        },
        TestCase {
            name: "Wikidata Integration".to_string(),
            description: "Test Wikidata meme integration".to_string(),
            category: TestCategory::MemeFeatures,
            test_fn: test_wikidata_integration,
        },
        // Lean Integration
        TestCase {
            name: "Expression Parsing".to_string(),
            description: "Test parsing Lean expressions".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: test_expression_parsing,
        },
        TestCase {
            name: "Emoji Conversion".to_string(),
            description: "Test converting expressions to emoji representation".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: test_emoji_conversion,
        },
        TestCase {
            name: "Level Management".to_string(),
            description: "Test Lean level operations".to_string(),
            category: TestCategory::LeanIntegration,
            test_fn: test_level_management,
        },
        // Notification System
        TestCase {
            name: "Notification Creation".to_string(),
            description: "Test creating notifications".to_string(),
            category: TestCategory::NotificationSystem,
            test_fn: test_notification_creation,
        },
        TestCase {
            name: "Timed Notifications".to_string(),
            description: "Test timed notification functionality".to_string(),
            category: TestCategory::NotificationSystem,
            test_fn: test_timed_notification,
        },
        // Connection Management
        TestCase {
            name: "Wallet Connection".to_string(),
            description: "Test wallet adapter connections".to_string(),
            category: TestCategory::ConnectionManagement,
            test_fn: test_wallet_connection,
        },
        TestCase {
            name: "Connection Filtering".to_string(),
            description: "Test connection filtering functionality".to_string(),
            category: TestCategory::ConnectionManagement,
            test_fn: test_connection_filtering,
        },
        // Utility Functions
        TestCase {
            name: "Data Parsing".to_string(),
            description: "Test data parsing utilities".to_string(),
            category: TestCategory::UtilityFunctions,
            test_fn: test_data_parsing,
        },
        TestCase {
            name: "Storage Operations".to_string(),
            description: "Test storage utility functions".to_string(),
            category: TestCategory::UtilityFunctions,
            test_fn: test_storage_operation,
        },
    ]
}

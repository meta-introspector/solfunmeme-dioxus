use component_registry_lib::ComponentName;
use std::collections::HashMap;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn get_emoji(
    component: &ComponentName,
    _props: Option<&HashMap<String, component_registry_lib::PropValue>>,
) -> &'static str {
    match component {
        ComponentName::StylingAndEmojis => "🎭",
        ComponentName::Header | ComponentName::MainApp => "🏠",
        ComponentName::ConnectWalletModalModal
        | ComponentName::NavWalletItem
        | ComponentName::ActiveAccountDropDown => "💸",
        ComponentName::PingCluster => "📡",
        ComponentName::PasswordApp
        | ComponentName::PasswordAppHeader
        | ComponentName::PasswordErrorMessage
        | ComponentName::LoginScreen
        | ComponentName::PasswordMainInterface
        | ComponentName::PasswordList
        | ComponentName::AddPasswordForm
        | ComponentName::PasswordDetail => "🔒",
        ComponentName::WelcomeScreen => "👋",
        ComponentName::Accounts
        | ComponentName::ClusterSuccess
        | ComponentName::TokenAccountCard
        | ComponentName::TxCard => "💳",
        ComponentName::Airdrop => "🎁",
        ComponentName::Clusters | ComponentName::ClusterInfo | ComponentName::AddClusterModal => "🌐",
        ComponentName::QueryCoinDialog => "🪙",
        ComponentName::ComponentMemeView
        | ComponentName::MemeCategoryView
        | ComponentName::ComponentMemeExplorer
        | ComponentName::Memes
        | ComponentName::MemeCardHeader
        | ComponentName::InputSection
        | ComponentName::ExpressionTypeSelector
        | ComponentName::ExpressionInputs
        | ComponentName::MetadataInputs
        | ComponentName::CreateButton
        | ComponentName::SearchInput
        | ComponentName::ExpressionList
        | ComponentName::ExpressionCard
        | ComponentName::CodeDisplay
        | ComponentName::ExpressionMetadata
        | ComponentName::SimilaritySection
        | ComponentName::VectorSpace
        | ComponentName::MemesFooter
        | ComponentName::WikidataMemeView
        | ComponentName::WikidataMemeExplorer
        | ComponentName::WorkflowStepView
        | ComponentName::WorkflowMemeView
        | ComponentName::WorkflowMemeExplorer => "😂",
        ComponentName::ConnectionButtons
        | ComponentName::CoreButtons
        | ComponentName::CryptoButtons
        | ComponentName::ManagementButtons
        | ComponentName::TransactionButtons => "🛠️",
        ComponentName::CryptoFrontendApp
        | ComponentName::CryptoAppHeader
        | ComponentName::CardHeader
        | ComponentName::InputField
        | ComponentName::TextAreaField
        | ComponentName::ActionButton
        | ComponentName::CryptoErrorMessage
        | ComponentName::SuccessMessage
        | ComponentName::EncryptionForm
        | ComponentName::DecryptionForm
        | ComponentName::Encryption => "🔐",
        ComponentName::Dashboard => "📊",
        ComponentName::Extras
        | ComponentName::SignMessage
        | ComponentName::SignTx
        | ComponentName::SignInWithSolana => "✨",
        ComponentName::Footer => "📍",
        ComponentName::GitParser2 => "📜",
        ComponentName::MemeManagement | ComponentName::MetaMemeOperations => "🎨",
        ComponentName::Notification | ComponentName::Notification2 => "🔔",
        ComponentName::PageNotFound => "❓",
        ComponentName::QueryAccountDialog => "🔍",
        ComponentName::ReceiveSol => "📥",
        ComponentName::SendSol => "📤",
        ComponentName::ConnectWalletFirst => "🔗",
    }
}

pub fn get_emoji_style(_component: &ComponentName, _props: &HashMap<String, component_registry_lib::PropValue>) -> &'static str {
    "text-gray-700"
}

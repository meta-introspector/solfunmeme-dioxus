// styles.rs - Centralized styling constants
// All CSS classes are defined here for easy maintenance and consistency

// Layout Styles
pub const APP_CONTAINER: &str = "min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 p-4 sm:p-8";
pub const MAIN_WRAPPER: &str = "max-w-6xl mx-auto";
pub const GRID_LAYOUT: &str = "grid grid-cols-1 xl:grid-cols-2 gap-6 lg:gap-8";

// Typography
pub const MAIN_TITLE: &str = "text-3xl sm:text-4xl font-bold text-center mb-8 text-gray-800 bg-gradient-to-r from-blue-600 to-indigo-600 bg-clip-text text-transparent";
pub const CARD_TITLE: &str = "text-2xl font-semibold mb-6 text-gray-800 border-b border-gray-200 pb-2";
pub const LABEL: &str = "block text-sm font-medium text-gray-700 mb-2";
pub const SUCCESS_TITLE: &str = "font-semibold mb-3 text-green-800";
pub const TEXT_CONTENT: &str = "whitespace-pre-wrap text-gray-700 leading-relaxed";

// Card Styles
pub const CARD: &str = "bg-white rounded-xl shadow-lg p-6 sm:p-8 border border-gray-200 hover:shadow-xl transition-shadow duration-300";

// Form Styles
pub const FORM_CONTAINER: &str = "space-y-6";
pub const INPUT: &str = "w-full px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-200 bg-gray-50 hover:bg-white";
//pub const TEXTAREA: &str
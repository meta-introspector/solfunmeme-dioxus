// ============================================================================
// STYLE MODULE - CSS Styles and Theme Constants
// ============================================================================

pub struct Theme {
    // Colors
    pub primary_gradient: &'static str,
    pub secondary_gradient: &'static str,
    pub background_color: &'static str,
    pub surface_color: &'static str,
    pub text_primary: &'static str,
    pub text_secondary: &'static str,
    pub accent_color: &'static str,
    pub error_color: &'static str,
    pub success_color: &'static str,
    
    // Spacing
    pub spacing_xs: &'static str,
    pub spacing_sm: &'static str,
    pub spacing_md: &'static str,
    pub spacing_lg: &'static str,
    pub spacing_xl: &'static str,
    
    // Border radius
    pub radius_sm: &'static str,
    pub radius_md: &'static str,
    pub radius_lg: &'static str,
    
    // Typography
    pub font_family_mono: &'static str,
    pub font_family_sans: &'static str,
    pub font_size_sm: &'static str,
    pub font_size_md: &'static str,
    pub font_size_lg: &'static str,
    pub font_size_xl: &'static str,
    
    // Shadows
    pub shadow_sm: &'static str,
    pub shadow_md: &'static str,
    pub shadow_lg: &'static str,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            // Colors
            primary_gradient: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            secondary_gradient: "linear-gradient(45deg, #667eea, #764ba2)",
            background_color: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
            surface_color: "rgba(255,255,255,0.95)",
            text_primary: "#333",
            text_secondary: "#666",
            accent_color: "#667eea",
            error_color: "#ff4757",
            success_color: "#2ed573",
            
            // Spacing
            spacing_xs: "5px",
            spacing_sm: "10px",
            spacing_md: "15px",
            spacing_lg: "20px",
            spacing_xl: "30px",
            
            // Border radius
            radius_sm: "8px",
            radius_md: "12px",
            radius_lg: "15px",
            
            // Typography
            font_family_mono: "'Fira Code', monospace",
            font_family_sans: "system-ui, -apple-system, sans-serif",
            font_size_sm: "14px",
            font_size_md: "16px",
            font_size_lg: "1.2rem",
            font_size_xl: "2.5rem",
            
            // Shadows
            shadow_sm: "0 2px 8px rgba(0,0,0,0.1)",
            shadow_md: "0 4px 16px rgba(0,0,0,0.1)",
            shadow_lg: "0 8px 32px rgba(0,0,0,0.1)",
        }
    }
}

pub static THEME: Theme = Theme {
    // Colors
    primary_gradient: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    secondary_gradient: "linear-gradient(45deg, #667eea, #764ba2)",
    background_color: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)",
    surface_color: "rgba(255,255,255,0.95)",
    text_primary: "#333",
    text_secondary: "#666",
    accent_color: "#667eea",
    error_color: "#ff4757",
    success_color: "#2ed573",
    
    // Spacing
    spacing_xs: "5px",
    spacing_sm: "10px",
    spacing_md: "15px",
    spacing_lg: "20px",
    spacing_xl: "30px",
    
    // Border radius
    radius_sm: "8px",
    radius_md: "12px",
    radius_lg: "15px",
    
    // Typography
    font_family_mono: "'Fira Code', monospace",
    font_family_sans: "system-ui, -apple-system, sans-serif",
    font_size_sm: "14px",
    font_size_md: "16px",
    font_size_lg: "1.2rem",
    font_size_xl: "2.5rem",
    
    // Shadows
    shadow_sm: "0 2px 8px rgba(0,0,0,0.1)",
    shadow_md: "0 4px 16px rgba(0,0,0,0.1)",
    shadow_lg: "0 8px 32px rgba(0,0,0,0.1)",
};

// ============================================================================
// COMPONENT STYLES
// ============================================================================

pub struct Styles;

impl Styles {
    // Container styles
    pub fn app_container() -> String {
        format!(
            "font-family: {}; background: {}; min-height: 100vh; padding: {};",
            THEME.font_family_mono,
            THEME.background_color,
            THEME.spacing_lg
        )
    }

    // Header styles
    pub fn header() -> String {
        "text-align: center; margin-bottom: 30px; color: white;".to_string()
    }

    pub fn header_title() -> String {
        format!(
            "font-size: {}; margin-bottom: {}; text-shadow: 2px 2px 4px rgba(0,0,0,0.3);",
            THEME.font_size_xl,
            THEME.spacing_sm
        )
    }

    pub fn header_subtitle() -> String {
        format!(
            "font-size: {}; opacity: 0.9;",
            THEME.font_size_lg
        )
    }

    // Section styles
    pub fn section() -> String {
        format!(
            "background: {}; border-radius: {}; padding: 25px; margin-bottom: {}; box-shadow: {};",
            THEME.surface_color,
            THEME.radius_lg,
            THEME.spacing_xl,
            THEME.shadow_lg
        )
    }

    pub fn section_title() -> String {
        format!(
            "margin-bottom: {}; color: {};",
            THEME.spacing_lg,
            THEME.text_primary
        )
    }

    // Input styles
    pub fn input() -> String {
        format!(
            "padding: {}; border: 2px solid #ddd; border-radius: {}; font-size: {};",
            THEME.spacing_sm,
            THEME.radius_sm,
            THEME.font_size_md
        )
    }

    pub fn textarea() -> String {
        format!(
            "padding: {}; border: 2px solid #ddd; border-radius: {}; font-size: {}; min-height: 60px; resize: vertical;",
            THEME.spacing_sm,
            THEME.radius_sm,
            THEME.font_size_md
        )
    }

    pub fn search_input() -> String {
        format!(
            "width: 100%; padding: {}; border: 2px solid #ddd; border-radius: {}; font-size: {};",
            THEME.spacing_sm,
            THEME.radius_sm,
            THEME.font_size_md
        )
    }

    // Button styles
    pub fn primary_button() -> String {
        format!(
            "padding: 12px 24px; background: {}; color: white; border: none; border-radius: {}; font-size: {}; font-weight: bold; cursor: pointer; transition: transform 0.2s;",
            THEME.secondary_gradient,
            THEME.radius_sm,
            THEME.font_size_md
        )
    }

    pub fn delete_button() -> String {
        format!(
            "background: {}; color: white; border: none; border-radius: 50%; width: 30px; height: 30px; cursor: pointer; font-size: {};",
            THEME.error_color,
            THEME.font_size_md
        )
    }

    // Layout styles
    pub fn grid_auto_fit(min_width: &str) -> String {
        format!(
            "display: grid; grid-template-columns: repeat(auto-fit, minmax({}, 1fr)); gap: {};",
            min_width,
            THEME.spacing_sm
        )
    }

    pub fn grid_auto_fill(min_width: &str) -> String {
        format!(
            "display: grid; grid-template-columns: repeat(auto-fill, minmax({}, 1fr)); gap: {};",
            min_width,
            THEME.spacing_lg
        )
    }

    pub fn flex_center() -> String {
        "display: flex; align-items: center; justify-content: center;".to_string()
    }

    pub fn flex_between() -> String {
        "display: flex; justify-content: space-between; align-items: center;".to_string()
    }

    pub fn flex_with_gap(gap: &str) -> String {
        format!("display: flex; align-items: center; gap: {};", gap)
    }

    // Card styles
    pub fn card() -> String {
        format!(
            "background: {}; border-radius: {}; padding: {}; box-shadow: {}; transition: transform 0.2s;",
            THEME.surface_color,
            THEME.radius_md,
            THEME.spacing_lg,
            THEME.shadow_md
        )
    }

    pub fn card_hover() -> String {
        format!(
            "{} hover: transform: translateY(-2px);",
            Self::card()
        )
    }

    // Code styles
    pub fn code_block() -> String {
        format!(
            "background: #f8f9fa; padding: {}; border-radius: {}; margin-bottom: {}; font-family: {}; font-size: {}; overflow-x: auto;",
            THEME.spacing_md,
            THEME.radius_sm,
            THEME.spacing_sm,
            THEME.font_family_mono,
            THEME.font_size_sm
        )
    }

    // Form styles
    pub fn form_grid() -> String {
        format!("display: grid; gap: {};", THEME.spacing_md)
    }

    pub fn radio_group() -> String {
        format!(
            "display: grid; grid-template-columns: repeat(auto-fit, minmax(120px, 1fr)); gap: {}; margin-bottom: {};",
            THEME.spacing_sm,
            THEME.spacing_lg
        )
    }

    pub fn radio_label() -> String {
        format!(
            "display: flex; align-items: center; gap: {}; cursor: pointer;",
            THEME.spacing_xs
        )
    }

    pub fn checkbox_label() -> String {
        format!(
            "display: flex; align-items: center; gap: {};",
            THEME.spacing_xs
        )
    }

    // Text styles
    pub fn text_primary() -> String {
        format!("color: {};", THEME.text_primary)
    }

    pub fn text_secondary() -> String {
        format!("color: {};", THEME.text_secondary)
    }

    pub fn text_accent() -> String {
        format!("color: {}; font-weight: 500;", THEME.accent_color)
    }

    pub fn text_white() -> String {
        "color: white; text-shadow: 1px 1px 2px rgba(0,0,0,0.3);".to_string()
    }

    // Utility styles
    pub fn margin_bottom(size: &str) -> String {
        format!("margin-bottom: {};", size)
    }

    pub fn padding(size: &str) -> String {
        format!("padding: {};", size)
    }

    pub fn font_weight_bold() -> String {
        "font-weight: bold;".to_string()
    }

    pub fn font_weight_medium() -> String {
        "font-weight: 500;".to_string()
    }

    pub fn cursor_pointer() -> String {
        "cursor: pointer;".to_string()
    }

    pub fn overflow_hidden() -> String {
        "overflow: hidden;".to_string()
    }

    pub fn text_overflow_ellipsis() -> String {
        "white-space: nowrap; overflow: hidden; text-overflow: ellipsis;".to_string()
    }

    // Responsive breakpoints (for future use)
    pub fn mobile() -> &'static str {
        "@media (max-width: 768px)"
    }

    pub fn tablet() -> &'static str {
        "@media (max-width: 1024px)"
    }

    pub fn desktop() -> &'static str {
        "@media (min-width: 1025px)"
    }
}
// macro.rs - Fixed procedural macro
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;
use syn::ItemFn;
use syn::parse::Parser;
// Re-export the McpConfig from the mcp module
use rrust_kontekst_base::McpConfig;
//use dioxus_logger::tracing::info;
use syn::{self, parse_macro_input, meta::ParseNestedMeta, Error};

/// Parse a string literal value from meta
fn parse_string_value(meta: ParseNestedMeta, target: &mut String) -> Result<(), Error> {
    if let Ok(value) = meta.value() {
        if let Ok(lit_str) = value.parse::<syn::LitStr>() {
            *target = lit_str.value();
            Ok(())
        } else {
            Err(meta.error("expected string literal"))
        }
    } else {
        Err(meta.error("expected string literal"))
    }
}

/// Parse a boolean literal value from meta
fn parse_bool_value(meta: ParseNestedMeta, target: &mut bool) -> Result<(), Error> {
    if let Ok(value) = meta.value() {
        if let Ok(lit_bool) = value.parse::<syn::LitBool>() {
            *target = lit_bool.value;
            Ok(())
        } else {
            Err(meta.error("expected bool literal"))
        }
    } else {
        Err(meta.error("expected bool literal"))
    }
}

/// Parse an integer literal value from meta
fn parse_int_value(meta: ParseNestedMeta, target: &mut i32) -> Result<(), Error> {
    if let Ok(value) = meta.value() {
        if let Ok(lit_int) = value.parse::<syn::LitInt>() {
            match lit_int.base10_parse() {
                Ok(value) => {
                    *target = value;
                    Ok(())
                }
                Err(_) => Err(meta.error("invalid integer literal"))
            }
        } else {
            Err(meta.error("expected int literal"))
        }
    } else {
        Err(meta.error("expected int literal"))
    }
}

/// Parse a float literal value from meta
// FIXME: suggested by AI , untested and unused
// fn parse_float_value(meta: ParseNestedMeta, target: &mut f32) -> Result<(), Error> {
//     if let Ok(value) = meta.value() {
//         if let Ok(lit_float) = value.parse::<syn::LitFloat>() {
//             match lit_float.base10_parse() {
//                 Ok(value) => {
//                     *target = value;
//                     Ok(())
//                 }
//                 Err(_) => Err(meta.error("invalid float literal"))
//             }
//         } else {
//             Err(meta.error("expected float literal"))
//         }
//     } else {
//         Err(meta.error("expected float literal"))
//     }
// }

/// Parse an array of parameters
fn parse_params_array(meta: ParseNestedMeta, target: &mut Vec<String>) -> Result<(), Error> {
    meta.parse_nested_meta(|nested_meta| {
        if let Ok(value) = nested_meta.value() {
            if let Ok(lit_str) = value.parse::<syn::LitStr>() {
                target.push(lit_str.value());
                Ok(())
            } else {
                Err(nested_meta.error("expected string literal in array"))
            }
        } else {
            Err(nested_meta.error("expected value in array"))
        }
    })
}

/// Generate the MCP handler function
fn generate_handler_function(
    config: &McpConfig,
    fn_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let mcp_handler_name = syn::Ident::new(
        &format!("{}_mcp_handler", fn_name),
        fn_name.span(),
    );
    let label = &config.label;
    
    quote! {
        fn #mcp_handler_name(
            params: serde_json::Value
        ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<serde_json::Value, rrust_kontekst_base::McpError>> + Send>> {
            Box::pin(async move {
                match #fn_name().await {
                    Ok(result) => {
                        Ok(serde_json::json!({
                            "content": [{
                                "type": "text",
                                "text": format!("Component '{}' executed successfully", #label)
                            }],
                            "result": result,
                            "isError": false
                        }))
                    },
                    Err(e) => {
                        Err(rrust_kontekst_base::McpError::ExecutionError(format!("Component '{}' failed: {}", #label, e)))
                    }
                }
            })
        }
    }
}

/// Generate the registration code
fn generate_registration(
    config: &McpConfig,
    fn_name: &syn::Ident,
) -> proc_macro2::TokenStream {
    let mcp_handler_name = syn::Ident::new(
        &format!("{}_mcp_handler", fn_name),
        fn_name.span(),
    );
    let register_fn_name = syn::Ident::new(
        &format!("register_{}", fn_name),
        fn_name.span(),
    );
    
    let tool_name = &config.tool_name;
    let label = &config.label;
    let emoji = &config.emoji;
    let description = &config.description;
    let menu_type = &config.menu_type;
    let visible = config.visible;
    let order = config.order;
    let mcp_enabled = config.mcp_enabled;
    let returns = &config.returns;
    let fn_name_str = fn_name.to_string();
    
    quote! {
        // Auto-registration function
        #[allow(non_snake_case)]
//	#[ctor::ctor]  
        pub fn #register_fn_name() {
            use rrust_kontekst_base::{McpToolInfo, register_mcp_tool};
	    use dioxus_logger::tracing::info;
//	    use std::sync::Once;  
//            static INIT: Once = Once::new();  
              
            //INIT.call_once(|| {
//		eprintln!("register MCP tool '{:?}':", #fn_name_str,);
//		info!("register: {:?}", #fn_name_str,);
//	    });
//            
            static TOOL_INFO :  McpToolInfo = McpToolInfo {
                component_name: #fn_name_str,
                tool_name: #tool_name,
                menu_type: #menu_type,
                label: #label,
                emoji: #emoji,
                description: #description,
                visible: #visible,
                order: #order,
                mcp_enabled: #mcp_enabled,
                parameters: &[], // Simplified for now
                returns: #returns,
            };
            
            if let Err(e) = register_mcp_tool(&TOOL_INFO, #mcp_handler_name) {
                eprintln!("Failed to register MCP tool '{}': {}", #tool_name, e);
            }

	    // Use inventory to auto-call registration
            //inventory::submit! {
	    //inventory::Registry::new(#tool_name, #register_fn_name)
            }

        }//quote    
}

/// Generate component metadata for introspection
fn generate_metadata(
    config: &McpConfig,
    fn_name: &syn::Ident,
    input_fn: &ItemFn,
) -> proc_macro2::TokenStream {
    let metadata_name = syn::Ident::new(
        &format!("{}_METADATA", fn_name.to_string().to_uppercase()),
        fn_name.span(),
    );
    
    let fn_name_str = fn_name.to_string();
    let tool_name = &config.tool_name;
    let description = &config.description;
    
    quote! {
        #[allow(non_upper_case_globals)]
        pub static #metadata_name: std::sync::OnceLock<serde_json::Value> = std::sync::OnceLock::new();
        
        pub fn get_metadata() -> &'static serde_json::Value {
            #metadata_name.get_or_init(|| {
                serde_json::json!({
                    "component_name": #fn_name_str,
                    "tool_name": #tool_name,
                    "description": #description,
                    "function_signature": stringify!(#input_fn),
                    "generated_at_compile_time": true
                })
            })
        }
    }
}

#[proc_macro_attribute]
pub fn mcp_component(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    //info!("got namel: {:?}", fn_name);
    // Parse the macro arguments
    let mut config = match parse_macro_args_helper(args) {
        Ok(config) => config,
        Err(err) => return err.to_compile_error().into(),
    };
    
    // Set default values based on function name if not provided
    if config.label.is_empty() {
        config.label = fn_name_str.clone();
    }
    if config.tool_name.is_empty() {
        config.tool_name = fn_name_str.to_lowercase().replace("component", "");
    }
    
    // Generate all the components
    let handler_function = generate_handler_function(&config, fn_name);
    let registration = generate_registration(&config, fn_name);
    let metadata = generate_metadata(&config, fn_name, &input_fn);
    
    // Combine everything
    let expanded = quote! {
        #input_fn
        
        #handler_function
        
        #registration
        
        #metadata
    };

    //eprintln!("TOKENS: {}", expanded);
    TokenStream::from(expanded)
}

// Helper function - Fixed to avoid lifetime issues
fn parse_macro_args_helper(args: TokenStream) -> syn::Result<McpConfig> {
    let mut config = McpConfig::default();
    
    // Convert to proc_macro2::TokenStream for parsing
    let args2: proc_macro2::TokenStream = args.into();
    
    let parser = syn::meta::parser(|meta: ParseNestedMeta| {
        let path_str = meta.path
            .get_ident()
            .map(|i| i.to_string())
            .unwrap_or_default();
            
        match path_str.as_str() {
            "menu" => parse_string_value(meta, &mut config.menu_type),
            "label" => parse_string_value(meta, &mut config.label),
            "emoji" => parse_string_value(meta, &mut config.emoji),
            "description" => parse_string_value(meta, &mut config.description),
            "tool_name" => parse_string_value(meta, &mut config.tool_name),
            "returns" => parse_string_value(meta, &mut config.returns),
            "visible" => parse_bool_value(meta, &mut config.visible),
            "mcp" => parse_bool_value(meta, &mut config.mcp_enabled),
            "order" => parse_int_value(meta, &mut config.order),
            "params" => parse_params_array(meta, &mut config.parameters),
            _ => {
                // Silently ignore unknown attributes instead of returning an error
                Ok(())
            }
        }
    });
    
    // Parse the TokenStream
    parser.parse2(args2)?;
    
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    // Example usage of the macro
    #[mcp_component(
        menu = "ai_tools",
        label = "Test Component", 
        emoji = "🧪",
        description = "A test component for demonstration",
        order = 1
    )]
    pub async fn test_component() -> Result<String, Box<dyn std::error::Error>> {
        Ok("Test component executed successfully".to_string())
    }
}

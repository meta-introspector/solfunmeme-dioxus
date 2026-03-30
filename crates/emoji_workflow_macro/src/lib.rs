use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro_attribute]
pub fn emoji_workflow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let emoji_string = parse_macro_input!(attr as LitStr).value();

    let input_fn = parse_macro_input!(item as syn::ItemFn);
    let fn_name = &input_fn.sig.ident;

    let register_fn_name = syn::Ident::new(
        &format!("register_workflow_for_{}", fn_name),
        fn_name.span(),
    );

    let expanded = quote! {
        #input_fn

        #[allow(non_snake_case)]
        #[cfg(not(target_arch = "wasm32"))]
        #[ctor::ctor]
        fn #register_fn_name() {
            emoji_lang_plugin::register_emoji_workflow(
                stringify!(#fn_name).to_string(),
                #emoji_string.to_string(),
            );
            println!("Registered workflow '{}' with emoji string: '{}'", stringify!(#fn_name), #emoji_string);
        }
    };

    expanded.into()
}

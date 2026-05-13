use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

/// Creates an Alert struct that implements Display and From for multiple error types.
///
/// Usage:
/// ```markdown
/// #[alert(errors = [MyError, AnotherError])]
/// pub struct Alert {
///     message: String,
/// }
/// ```
#[proc_macro_attribute]
pub fn alert(args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let struct_vis = &input.vis;

    // Parse the error types from the attribute
    let attr_parser = parse_macro_input!(args as syn::MetaNameValue);
    let error_types = match attr_parser.value {
        syn::Expr::Array(array) => array
            .elems
            .iter()
            .map(|expr| match expr {
                syn::Expr::Path(path) => path.path.clone(),
                _ => panic!("Expected type paths in errors list"),
            })
            .collect::<Vec<_>>(),
        _ => panic!("Expected array syntax: errors = [Type1, Type2, ...]"),
    };

    // Generate the From implementations
    let from_impls = error_types.iter().map(|error_type| {
        quote! {
            impl From<#error_type> for #struct_name {
                fn from(value: #error_type) -> Self {
                    #struct_name {
                        message: format!("{}: {}", stringify!(#error_type), value),
                    }
                }
            }
        }
    });

    // Generate the struct with derives and Display implementation
    let expanded = quote! {
        #[derive(Debug, Clone, PartialEq)]
        #struct_vis struct #struct_name {
            message: String,
        }

        impl std::fmt::Display for #struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                writeln!(f, "{}", self.message)
            }
        }

        #(#from_impls)*
    };

    TokenStream::from(expanded)
}

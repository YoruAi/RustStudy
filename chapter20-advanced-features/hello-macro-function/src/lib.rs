use proc_macro::TokenStream;
use quote::quote;

// function macro
// need to declare the lib crate [lib] proc-macro = true in Cargo.toml
#[proc_macro] // called when hello_macro_function!(input)
pub fn hello_macro_function(input: TokenStream) -> TokenStream {
    let input_string = input.to_string();

    impl_hello_macro(&input_string)
}

fn impl_hello_macro(value: &str) -> TokenStream {
    let expanded = quote! {
        println!("{}", #value);
    };
    TokenStream::from(expanded)
}

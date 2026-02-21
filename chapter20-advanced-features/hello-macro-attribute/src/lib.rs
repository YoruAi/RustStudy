use proc_macro::TokenStream; // proc_macro is auto imported
use quote::quote;
use syn::{ItemFn, LitBool, parse_macro_input};

// attribute macro
// need to declare the lib crate [lib] proc-macro = true in Cargo.toml
#[proc_macro_attribute] // called when #[hello_macro_attribute(attr)]
pub fn hello_macro_attribute(attr: TokenStream, item: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate.
    // See more info at syn's documentation
    let attr_bool = parse_macro_input!(attr as LitBool);
    let attr_value = attr_bool.value;
    let mut func = parse_macro_input!(item as ItemFn);

    impl_hello_macro(attr_value, &mut func)
}

fn impl_hello_macro(attr_value: bool, func: &mut ItemFn) -> TokenStream {
    let insert_code = quote! {
        println!("[hello_macro_attribute] attr: {}", #attr_value);
    };

    let original_body = &func.block;
    func.block = Box::new(syn::parse_quote! {
        {
            #insert_code
            #original_body
        }
    });

    TokenStream::from(quote! {
        #func
    })
}

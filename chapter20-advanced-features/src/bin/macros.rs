fn main() {
    // macro: declarative macro and 3 procedural macro(derive, attribute, function)
    // macros are a way of writing code that writes other code(metaprogramming)
    // macros can take a variable number of parameters

    // [declarative macro]
    // use macro_rules!
    // more info: https://doc.rust-lang.org/reference/macros-by-example.html
    // or: https://veykril.github.io/tlborm/
    my_vec![1, 2, 3];

    // [procedural macro]
    // the procedural macro definitions must reside in their own crate with a special crate type.

    // 1. derive macro
    // see hello-macro-derive crate
    use chapter20_advanced_features::HelloMacro;
    use hello_macro_derive::HelloMacro;
    #[derive(HelloMacro)]
    struct Pancakes;
    Pancakes::hello_macro();

    // 2. attribute macro
    // see hello-macro-attribute crate
    use hello_macro_attribute::hello_macro_attribute;
    #[hello_macro_attribute(true)]
    fn hello_macro() {
        println!("Hello, Macro! My macro!");
    }
    hello_macro();

    // 3. function macro
    // see hello-macro-function crate
    use hello_macro_function::hello_macro_function;
    hello_macro_function!(HELLO MACRO);
}

// declarative macro
#[macro_export] // means that this macro can be used in other crate
macro_rules! my_vec {
    // pattern match
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

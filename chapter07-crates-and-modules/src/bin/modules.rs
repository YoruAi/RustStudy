fn main() {
    println!("[compile]");
    println!("crate root: src/lib.rs or src/main.rs");
    println!("declare new module: inline, src/new_mod.rs or src/new_mod/mod.rs");
    println!("declare submodule: inline, src/new_mod/submodule.rs or src/new_mod/submodule/mod.rs");
    println!("you should use one style of `a.rs and a/mod.rs` in one module");
    println!("the path to the code refer: crate::new_mod::submodule");
    println!("code within a module is private from its parent modules, should pub mod and items");
    println!("use crate::new_mod::submodule::Foo shortcut");
}
fn main() {
    println!("[crate]");
    println!("Library crate & Binary crate");
    println!("A crate is the smallest amount of code that the Rust compiler considers at a time.");
    println!("[package]");
    println!("A package contains a Cargo.toml file that describes how to build those crates.");
    println!("A package can contain as many binary crates, but at most only one library crate.");
    println!("At least one crate certainly.");
    println!("`main.rs` is the crate root of a binary crate with the same name as the package;");
    println!("`lib.rs` is the crate root of a library crate with the same name as the package;");
    println!("multiple binary crates in the src/bin directory.");
}

fn main() {
    println!("[workspace]");
    println!("resolver = \"3\"");
    println!("members = [\"member1\", \"member2\"]");
    println!("then you can add path dependencies in package Cargo.toml");

    println!("cargo test --workspace");
    println!("cargo test -p crate_name");
    println!("use `-p` flag and specifying the name of the crate");
}

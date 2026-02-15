fn main() {
    println!("cargo build --release");
    // Cargo.toml
    println!(
        "[profile.dev]
opt-level = 0"
    );
    println!(
        "[profile.release]
opt-level = 3"
    );
}

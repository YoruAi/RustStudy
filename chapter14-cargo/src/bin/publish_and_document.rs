//! # Documentation comments
//!
//! this **crate** is for documentation comments and publish example.
//! this is contained item comments, usually used to document your crate at crate root.

#![allow(unused_imports)]

fn main() {
    println!("[documentation comments]");
    println!("cargo doc --open");
    println!("commonly used sections: Examples, Panics, Errors, Safety(if unsafe)");
    println!("cargo test will also test codes in document");

    println!("[publish]");
    println!("cargo login, and enter crates.io api token(.cargo/credentials");
    println!("add meta date under [package] in Cargo.toml");
    println!("crate name: name=\"name\"");
    println!("crate license: license=\"MIT OR Apache-2.0\"");
    println!("crate version, edition, description and so on.");
    println!("see more in cargo doc: https://doc.rust-lang.org/cargo/");
    println!("be careful: publishing is permanent, that version can't be overwritten or deleted");
    println!("cargo publish");

    println!("[yank]");
    println!(
        "Yanking prevents new projects from depending on that version \
    while allowing all existing projects that depend on it to continue(Cargo.lock)."
    );
    println!("cargo yank --vers 0.1.0"); // yank
    println!("cargo yank --vers 0.1.0 --undo"); // undo yank
}

// re-exports
pub use self::utils::add_one;

pub mod utils {
    /// Adds one to the number given.
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> i32 {
        x + 1
    }
}

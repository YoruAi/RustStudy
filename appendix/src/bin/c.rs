fn main() {
    println!("[Derivable Traits]");
    println!("https://doc.rust-lang.org/book/appendix-03-derivable-traits.html");

    println!("All the traits in the standard library that you can use with derive:");
    println!("Debug: use `{{:?}}`");
    println!(
        "PartialEq and Eq: impl ==, != and eq. \
        PartialEq means some signed values may not be eq(like NaN)"
    );
    println!(
        "PartialOrd and Ord: impl <, >, <=, >=. Must impl PartialEq and Eq. \
        PartialOrd impl partial_cmp->Option<Ordering> and Ord impl cmp->Ordering. \
        PartialOrd means some values may not be ordered(like NaN, partial_cmp will return None)"
    );
    println!("Clone: impl clone, which use deep copy");
    println!(
        "Copy: allows you to duplicate a value by only copying bits stored on the stack. \
        Must impl Clone"
    );
    println!("Hash: impl hash");
    println!("Default: impl default. Can use ..Default::default() to set the rest of the fields");

    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash, Default)]
    struct Point {
        x: i32,
        y: i32,
    }
}

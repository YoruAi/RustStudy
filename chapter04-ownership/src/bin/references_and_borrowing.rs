fn main() {
    let mut s1 = String::from("hello");

    // mutable reference(only one and no immutable reference)
    let r1 = &mut s1;
    change(r1);
    // a reference’s scope starts from where it is introduced
    // and continues through the last time that reference is used
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // [conclusion]
    // At any given time, you can have either one mutable reference
    // or any number of immutable references.
    // References must always be valid.
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn main() {
    println!("[RULE]");
    println!("Each value in Rust has an owner.");
    println!("There can only be one owner at a time.");
    println!("When the owner goes out of scope, the value will be dropped.");

    println!("[example] String");
    {
        let mut s = String::from("ownership");
        s.push_str(", world");
        // drop(s)
        let mut s = String::from("hello");
        s.push_str(", world");
        // drop(s);
    }
    let x: i32 = 5;
    let y = x; // copy(stack)
    let s1 = String::from("hello");
    let s2 = s1; // move(prevents double-free or use-after-free)
    let _s3 = s2.clone(); // copy
    takes_ownership(s2); // move
    makes_copy(y); // copy
}

fn takes_ownership(some_string: String) {
    println!("{some_string}");
    // drop(some_string);
    // or you can give back ownership by return it
}

fn makes_copy(some_integer: i32) {
    println!("{some_integer}");
}

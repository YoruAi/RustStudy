fn main() {
    println!("statements and expressions");
    println!(
        "Calling a function or macro, \
    and a new scope block created with curly brackets are all expressions"
    );
    let _y = {
        let x = 3;
        x + 1
    };
    let _x = plus_one(5);
}

// snake case
fn plus_one(x: i32) -> i32 {
    x + 1
}

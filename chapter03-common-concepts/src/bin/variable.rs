fn main() {
    println!("Variables are default immutable(or immutability error).");
    let x = 5;
    println!("x = {}", x);
    let mut y = 6;
    println!("y = {}", y);
    y = 7;
    println!("y = {}", y);

    println!("The type of the constants must be annotated.");
    const Z: i32 = 8;
    println!("Z = {}", Z);

    println!("Shadowing");
    {
        let x = (x + 1).to_string();
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");
}

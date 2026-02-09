fn main() {
    println!("scalar, compound");
    println!("statically typed");
    println!("[scalar]");
    println!("integer: i8, u8,..., i128, u128, isize, usize");
    let _x = 12_345;
    let _x = 0x8badf00du32;
    let _x = 0o77;
    let _x = 0b1111_0000;
    let _x = b'A';
    println!("float: f32, f64");
    println!("boolean: bool");
    println!("character: char");
    let _z: char = 'ℤ';
    println!("[compound]");
    println!("tuple: (i32, char, ...)");
    let _unit = (); // unit
    let tup: (i32, char) = (500, 'z');
    let (_x, _y) = tup; // pattern matching to destructure
    let _x = tup.0;
    let _y = tup.1;
    println!("array: [i32; 5]");
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let _a = [3; 5]; // repeat
}

fn main() {
    let mut v: Vec<i32> = Vec::new(); // omitted type if compiler can infer
    let mut v = vec![1, 2, 3];

    v.push(4);
    v.pop();

    let third = v.get(2); // will return None if out of bounds
    let third = &v[2]; // will panic if out of bounds
    // v.pop(); // immutable reference exists
    println!("{third}");

    for i in &v {
        println!("{i}");
    }
    for i in &mut v {
        *i += 50;
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    // drop vector and drop the elements inside.
}

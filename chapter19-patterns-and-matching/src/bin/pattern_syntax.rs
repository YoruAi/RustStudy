fn main() {
    // 1. Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        _ => println!("anything"),
    }

    // 2. Matching Named Variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // shadows
        _ => println!("Default case, x = {x:?}"),
    }
    println!("at the end: x = {x:?}, y = {y}");

    // 3. Matching Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"), // or
        3 => println!("three"),
        _ => println!("anything"),
    }

    // 4. Matching Ranges of Values with ..=
    let x = 5;
    match x {
        // only char and number
        1..=5 => println!("one through five"),
        6..10 => println!("five through nine"),
        _ => println!("something else"),
    }

    // 5. Destructuring to Break Apart Values
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    let Point { x, y } = p; // shorthand
    match p {
        // stop when matching
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    // 6. Enums & Nested Structs and Enums
    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }
    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {s}, value {v}");
        }
    }

    // 7. Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // 8. Ignoring Values in a Pattern
    fn foo(_: i32, y: i32) {
        // use `_` to not bind the value(won't even move)
        let _x = 5; // unused variable(will move)
        println!("This code only uses the y parameter: {y}");
    }
    let (a, .., d) = (1, 2, 3, 4); // ignore remaining parts of a value(unambiguous)

    // 9. Adding Conditionals with Match Guards
    let num = Some(4);
    match num {
        // can be used only in match expression
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"), // solve shadow
        _ => println!("Default case, x = {x:?}"),
    }

    // 10. @ Binding
    enum Greeting {
        Hello { id: i32 },
    }
    let msg = Greeting::Hello { id: 5 };
    match msg {
        Greeting::Hello { id: id @ 3..=7 } => println!("Found an id in range: {id}"),
        Greeting::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Greeting::Hello { id } => println!("Found some other id: {id}"),
    }
}

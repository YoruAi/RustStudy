// enum
enum IpAddrKind {
    V4,
    V6,
}
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
// one of them is same as a struct
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    // methods
    fn call(&self, x: i32) {
        // pattern matches are exhaustive
        match self {
            Message::Quit => {}
            Message::Move { x, y } => {
                println!("x: {x}, y: {y}");
            }
            Message::Write(message) => {
                println!("{message}");
            }
            Message::ChangeColor(_, _, _) => {}
        }

        match x {
            42 => {
                println!("42!");
            }
            // variable match
            i if i < 42 => {
                println!("less than 42");
            }
            _ => {} // wildcard
        }
    }
}

fn main() {
    route(IpAddr::V4(127, 0, 0, 1));
    route(IpAddr::V6(String::from("::1")));

    let m = Message::Write(String::from("hello"));
    m.call(42);

    // Option to prevent null
    let some_string = Some(String::from("world"));
    let absent_number: Option<i32> = None;
    let _ = match absent_number {
        None => None,
        Some(i) => Some(i + 1),
    };

    // if-let
    let _ = if let Some(s) = &some_string {
        println!("{s}");
        s
    } else {
        println!("None");
        return; // will return from outer function
    };
    // let-else
    let Some(s) = &some_string else {
        println!("None");
        return; // must return
    };
    println!("{s}");
}

fn route(ip: IpAddr) {
    println!("{:?}", ip);
}

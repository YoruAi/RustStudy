// struct
#[derive(Debug)] // impl Debug
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl User {
    // methods
    fn sign_in(&mut self) {
        self.active = true;
        self.sign_in_count += 1;
    }

    // `&self` means `self: &Self`
    fn username(&self) -> &str {
        &self.username
    }
}

impl User {
    // associated functions
    fn new(email: String, username: String) -> Self {
        Self {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

// tuple struct
struct Point(i32, i32, i32);

// unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        email: String::from("<EMAIL>"),
        username: String::from("username123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("username123@example.com");

    let user1 = build_user(
        String::from("username123@example.com"),
        String::from("username123"),
    );
    println!("{}", user1.active);
    println!("{}", user1.username);
    println!("{}", user1.email);
    println!("{}", user1.sign_in_count);
    println!("{user1:#?}"); // Debug print format
    dbg!(&user1);
    let mut user2 = User {
        email: String::from("another@example.com"),
        ..user1 // end, move the rest(actually `username`)
    };

    let origin = Point(0, 0, 0);
    let Point(_x, _y, _z) = origin;

    let _always_equal = AlwaysEqual;

    user2.sign_in();
    let _ = user2.username();
    let _ = User::new(
        String::from("username123@example.com"),
        String::from("username123"),
    );
}

fn build_user(email: String, username: String) -> User {
    User {
        // field init shorthand
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

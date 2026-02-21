use std::fmt::{self, Display, Formatter};
use std::ops::Add;

fn main() {
    // [Associated Types]
    // we won't impl different types traits unlike generic types
    pub trait Iterator {
        type Item; // associated type
        fn next(&mut self) -> Option<Self::Item>;
    }

    // [Default Generic Parameters and Operator Overloading]
    // operator overloading in std::ops like Add
    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }
    // Add<Rhs=Self> default generic parameters
    impl Add for Point {
        type Output = Point;
        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    // newtype pattern
    struct Millimeters(u32);
    struct Meters(u32);
    impl Add<Meters> for Millimeters {
        type Output = Millimeters;
        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    // [Disambiguating Between Identically Named Methods]
    trait Pilot {
        fn fly(&self);
    }
    trait Wizard {
        fn fly(&self);
    }
    struct Human;
    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }
    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }
    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }
    let person = Human;
    person.fly();
    Pilot::fly(&person);
    Wizard::fly(&person);
    // fully qualified syntax
    trait Animal {
        fn baby_name() -> String;
    }
    struct Dog;
    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }
    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }
    println!("A baby dog is called a {}", Dog::baby_name()); // Spot
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name()); // puppy

    // [Supertraits]
    trait OutlinePrint: Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {output} *");
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }
    impl Display for Point {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    // [Implementing External Traits with the Newtype Pattern]
    // use newtype pattern to get around the orphan rule restriction
    struct Wrapper(Vec<String>);
    impl Display for Wrapper {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "Wrapper:[{}]", self.0.join(", "))
        }
    }
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}

fn main() {
    println!("generic");

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    let p = Point::<i32, f64> { x: 10, y: 10.0 }; // or omit if compiler can infer
    p.y(); // only U = f64
}

// generic function
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// generic struct/enum
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
    fn mix<T1, U1>(self, other: Point<T1, U1>) -> Point<T, U1> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}
// constraint
impl<T> Point<T, f64> {
    fn y(&self) -> &f64 {
        println!("f64");
        &self.y
    }
}

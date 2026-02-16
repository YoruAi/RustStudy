fn main() {
    println!("Deref trait customize the behavior of dereference operator *");
    let mut x = MyBox::new(5);
    *x = 3;
    println!("x is {}", *x); // same as *(x.deref()) only deref once

    println!(
        "it happens automatically when passing a ref to a value as an argument \
     that doesn’t match the parameter type in the function or method definition. "
    );
    let m = MyBox::new(String::from("Rust"));
    fn hello(name: &str) {}
    hello(&m); // &MyBox<String> -> &String -> &str, same as hello(&(*m)[..])

    println!("[conclusion] rust does deref coercion when:");
    println!("1. from &T to &U when T: Deref<Target=U>");
    println!("2. from &mut T to &mut U when T: DerefMut<Target=U>");
    println!("3. from &mut T to &U when T: Deref<Target=U>");
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::{Deref, DerefMut};
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

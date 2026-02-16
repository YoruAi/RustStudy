use std::rc::Rc;

fn main() {
    println!("reference counting pointer");
    println!("Rc<T> allows a single value to have multiple owners");
    println!("cannot borrow data in an `Rc` as mutable");
    println!("be careful: only single-threaded scenarios");

    enum List {
        Cons(i32, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Nil)));
    // strong_count + 1
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
        // auto drop
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

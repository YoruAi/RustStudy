fn main() {
    println!("boxes allow you to store data on the heap");
    println!("box impl Deref, Drop trait");
    let mut b = Box::new(5);
    *b = 6;
    println!("b = {b}");
    println!("box is sized");
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}

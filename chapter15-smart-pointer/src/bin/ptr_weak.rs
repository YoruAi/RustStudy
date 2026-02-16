use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    println!("reference cycle will cause memory leak");
    println!("generally caused by the combination of Rc<T> and RefCell<T>");

    // reference cycle
    #[derive(Debug)]
    enum List {
        Cons(i32, RefCell<Rc<List>>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));
    if let Cons(_, link) = &*a {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("weak reference won't decide whether the object will destruct");
    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    // upgrade
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    // downgrade
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    // drop(branch); // branch will be dropped successfully
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}

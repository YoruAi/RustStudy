use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    println!(
        "interior mutability: unsafe to mutate data \
    even when there are immutable references to that data"
    );
    println!("RefCell<T> type represents single ownership over the data");
    println!("with RefCell<T>, these invariants are enforced at runtime(panic, not compile error)");
    println!("be careful: only single-threaded scenarios");

    println!("RefCell<T> keeps track of how many Ref<T> and RefMut<T> are currently active");
    let ref_cell_vec: RefCell<Vec<String>> = RefCell::new(vec![]);
    let mut ref_mut = ref_cell_vec.borrow_mut();
    ref_mut.push("hello".to_string()); // RefMut<T>
    drop(ref_mut); // otherwise panic
    println!("{:?}", ref_cell_vec.borrow()); // Ref<T>

    // mutable, unlike Rc which cannot mutate the node data
    #[derive(Debug)]
    enum List {
        Cons(RefCell<i32>, Rc<List>),
        Nil,
    }
    use List::{Cons, Nil};
    let a = Rc::new(Cons(RefCell::new(5), Rc::new(Nil)));
    let b = Cons(RefCell::new(3), Rc::clone(&a));
    let c = Cons(RefCell::new(4), Rc::clone(&a));
    if let Cons(cell_value, _) = &*a {
        *cell_value.borrow_mut() += 10;
    }
    println!("a after = {a:?}");
    println!("b after = {b:?}");
    println!("c after = {c:?}");
}

pub trait Messenger {
    fn send(&self, msg: &str);
}
pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}
impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }
    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }
    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);
        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

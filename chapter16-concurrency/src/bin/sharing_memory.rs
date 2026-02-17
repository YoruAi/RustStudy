use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    println!("mutex(mutual exclusion), lock, guarding");
    let m = Mutex::new(5);
    {
        // blocking until lock is available
        // If another user of this mutex panicked while holding the mutex,
        // then this call will return an error once the mutex is acquired.
        let mut num = m.lock().unwrap();
        // MutexGuard is a smart pointer
        *num = 6;
        // drop and auto release the lock
    }
    println!("m = {m:?}");

    // Arc<T> instead Rc<T>, Atomics
    println!("use Arc<T> to share mutex");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
    // it's better to use AtomicI32 here

    println!("Mutex<T> provides interior mutability, as the Cell family does");
    println!("like Rc<T>/RefCell<T>, Mutex<T> comes with the risk of creating deadlocks");
}

use std::thread;
use std::time::Duration;

fn main() {
    println!("threads will cause race conditions, deadlocks, special bugs");
    println!("rust impl threads with 1:1 model");

    // thread::spawn -> JoinHandle<T>
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            // thread::sleep
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
    // join() to blocking
    handle.join().unwrap();

    // move
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}

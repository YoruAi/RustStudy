fn main() {
    // This part is suggested to be read from the original book.
    // https://doc.rust-lang.org/book/ch17-05-traits-for-async.html
    println!("[Future trait]");
    println!("type Output, fn poll -> Poll<Output>, Poll<T>{{Ready(T), PENDING}}");
    println!("a runtime polls each future, put the future back to sleep when it is not ready.");

    println!("[Pin/Unpin trait]");
    println!("make sure the data structure doesn’t move in memory");
    println!("we only need to think about pinning when items have internal references");
    println!("Unpin is a marker trait, auto impl by compiler");
    println!("Unpin is to tell the compiler that it’s fine to move items around");
    println!(
        "impl !Unpin for SomeType(like future), that need to uphold those guarantees to be safe \
    whenever a pointer to that type is used in a Pin."
    );

    println!("[Stream trait]");
    println!("Stream combines Iterator(Option) and Future(Poll)");
    println!("fn poll_next -> Poll<Option<Item>>");
    println!("StreamExt trait: impl next() for default poll_next()");

    // [key flow]
    // 1. Async Block → Future
    // Every async block creates a Future object
    // The compiler generates an enum state machine
    // Each await point becomes one state variant
    // The state machine saves data needed after each await

    // 2. Poll Drives the State Machine
    // Future implements the poll() method
    // poll() is the async code transformed by compiler
    // Each poll() call executes until the next await
    // At await, it returns Pending and gives control back

    // 3. Pin Prevents Move
    // Moving a Future = moving the state machine enum
    // The state machine may have self-references
    // Example: a field points to another field in the same struct
    // Moving breaks these references (dangling pointer)
    // Pin locks the memory location, prevents moving
}

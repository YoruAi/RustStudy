use std::net::IpAddr;

fn main() {
    println!("recoverable and unrecoverable errors");

    // What are panics?
    println!("panics are unrecoverable errors");
    println!("these panics will print a failure message, unwind, clean up the stack, and quit");
    println!("unwinding means Rust walks back up the stack and cleans up data from each function");

    println!("use `panic = 'abort'` in Cargo.toml to switch from unwinding to aborting");
    println!("use `RUST_BACKTRACE=1` environment to get backtrace");

    // When to use panic?
    println!("use panic in examples, prototype code, and tests");
    println!("use panic when you have more information than the compiler");
    let home: IpAddr = "127.0.0.1"
        .parse()
        .expect("Hardcoded IP address should be valid");
    println!("use panic when your code could end up in a bad state");

    // How to panic?
    println!(
        "by taking an action that causes our code to panic \
    or explicitly calling the panic! macro"
    );
    // 1. cause panic
    let v = vec![1, 2, 3];
    println!("{}", v[99]);
    // 2. call the panic! macro
    panic!("crash and burn");
}

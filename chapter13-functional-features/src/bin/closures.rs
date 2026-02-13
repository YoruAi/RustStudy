use std::thread;

fn main() {
    println!("closures are anonymous functions you can save in a variable");
    let closure = |num: u32| -> u32 { num };
    let closure = |x| x;
    // compiler auto infer the type `fn(String) -> String`
    let s = closure(String::from("hello"));
    // let n = closure(5); // closure can't be used with different types

    println!("closures can capture values like function parameter");
    let mut list = vec![1, 2, 3];
    let mut borrows_mutably = || list.push(7);
    // here `list` is borrowed mutably, so it can't be borrowed again
    // println!("Before calling closure: {list:?}");
    borrows_mutably();

    println!("use `move` before to get the ownership forcibly");
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    println!(
        "the way a closure captures and handles values from the environment \
        affects which traits the closure implements"
    );
    println!(
        "traits are how functions and structs can specify what kinds of closures they can use"
    );
    println!(
        "FnOnce: every closure must impl, \
    a closure that moves captured values out of its body will only implement FnOnce"
    );
    println!(
        "FnMut: the closure can be call more than once, \
    applies to closures that don’t move captured values out but might mutate them"
    );
    println!(
        "Fn: the closure can be called more than once without mutating their environment, \
    applies to closures that don’t move captured values out and don’t mutate them, or capture nothing"
    );
}

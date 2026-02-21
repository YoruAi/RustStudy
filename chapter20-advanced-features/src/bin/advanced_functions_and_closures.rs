fn main() {
    // [Function Pointers]
    // fn impl Fn, FnMut, FnOnce, same as closures
    // C function accept function pointer only
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
    let answer = do_twice(add_one, 5);
    enum Status {
        Value(u32),
        Stop,
    }
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    // [Returning Closures]
    // closures are traits, we can use impl Trait
    fn returns_closure() -> impl Fn(i32) -> i32 {
        |x| x + 1
    }
    fn returns_init_closure(init: i32) -> impl Fn(i32) -> i32 {
        move |x| x + init
    }
    // not the same closures, we can use Box<dyn Trait>
    fn returns_closure_dyn() -> Box<dyn Fn(i32) -> i32> {
        Box::new(|x| x + 1)
    }
    fn returns_init_closure_dyn(init: i32) -> Box<dyn Fn(i32) -> i32> {
        Box::new(move |x| x + init)
    }
    let handlers = vec![returns_closure_dyn(), returns_init_closure_dyn(123)];
    for handler in handlers {
        let output = handler(5);
        println!("{output}");
    }
}

fn main() {
    // [Type Safety and Abstraction with the Newtype Pattern]
    struct Millimeters(u32);

    // [Type Synonyms and Type Aliases]
    type Kilometers = u32; // synonym for u32, not a new type
    type Result<T> = std::result::Result<T, std::io::Error>;
    type Thunk = Box<dyn Fn(i32) -> bool + Send + 'static>;
    let f: Thunk = Box::new(|_| true);

    // [The Never Type that Never Returns]
    fn bar() -> ! {
        panic!("This function never returns!");
    }
    loop {
        let num: u32 = match "err".trim().parse() {
            Ok(num) => num,  // u32
            Err(_) => break, // !
        };
    }
    // let x: ! = loop {};

    // [Dynamically Sized Types and the Sized Trait]
    // dynamically sized types, like str, need to be used with a pointer
    // auto impl Sized trait
    fn generic_sized<T>(t: T) {} // T: Sized
    fn generic<T: ?Sized>(t: &T) {} // only Sized can be used with ?, T can be Sized or not Sized
}

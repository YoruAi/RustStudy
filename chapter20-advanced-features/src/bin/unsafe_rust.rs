fn main() {
    // [unsafe superpowers]

    // 1. Dereferencing a Raw Pointer.
    // `*const T` and `*mut T`
    // here immutable means the pointer can’t be directly assigned to after being dereferenced.
    // raw pointers:
    // - Are allowed to ignore the borrowing rules
    // - Aren’t guaranteed to point to valid memory
    // - Are allowed to be null
    // - Don’t implement any automatic cleanup
    // we can create raw pointers in safe code:
    let mut num = 5;
    let r1 = &raw const num;
    let r2 = &raw mut num;
    let address = 0x012345usize;
    let r = address as *const i32;
    // but dereference raw pointers is unsafe
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    // 2. Call an unsafe function or method.
    /// # Safety
    /// ...
    unsafe fn dangerous() {}
    unsafe {
        dangerous();
    }
    // use `extern` to create/use foreign function interface(FFI)
    // use C ABI
    unsafe extern "C" {
        safe fn abs(input: i32) -> i32; // safe
    }
    println!("Absolute value of -3 according to C: {}", abs(-3));
    #[unsafe(no_mangle)] // no_mangle: prevent name mangling
    pub extern "C" fn call_from_c() {
        // use C ABI, so we can call it from C
        println!("Just called a Rust function from C!");
    }

    // 3. Access or modify a mutable static variable.
    static mut COUNTER: u32 = 0; // 'static, fixed address
    unsafe {
        COUNTER += 1;
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    // 4. Implement an unsafe trait.
    unsafe trait Foo {}
    unsafe impl Foo for i32 {}

    // 5. Access fields of `union`s.
    union Union {
        a: u32,
        b: u16,
    }
    let u = Union { a: 0x01234567 };
    unsafe {
        println!("b is: {}", u.b);
    }

    // [miri]
    // miri is a dynamic analysis tool in nightly rust
    // rustup +nightly component add miri
    // cargo +nightly miri run
}

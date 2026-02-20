fn main() {
    println!("paths for referring");
    println!("absolute and relative paths");
    println!(
        "Items in a parent module can’t use the private items inside child modules, \
    but items in child modules can use the items in their ancestor modules."
    );
    println!("exposing paths with `pub`");

    println!("paths tree is suggested to be defined in src/lib.rs");

    parent();
}

mod inner {
    pub mod sub_inner {
        pub fn func() {
            super::sub_func();
            let _ = SubInner { x: 42, y: 0 };
        }

        pub struct SubInner {
            pub x: i32,
            y: i32, // private
        }
        impl SubInner {
            pub fn new(x: i32) -> Self {
                Self { x, y: 0 }
            }
        }

        pub enum SubInnerEnum {
            // needn't to use `pub` here
            A,
            B,
        }
    }

    pub fn sub_func() {
        println!("inner sub_func");

        // cant access to y
        sub_inner::SubInner::new(42);
    }
}

// Same level as `inner` definition
fn parent() {
    inner::sub_func();
    inner::sub_inner::func();
}

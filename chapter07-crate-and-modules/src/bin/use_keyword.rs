fn main() {
    println!("`use` means `bring paths into scope`");
    // `use` should be in same scope
    use inner::sub_inner::sub_sub_inner;
    sub_sub_inner::func();

    use inner::sub_inner::SubInner;
    let _ = SubInner;
    let _ = sub_sub_inner::SubInner;

    use inner::sub_inner::sub_sub_inner::SubInner as SubSubInner;
    let _ = SubSubInner;

    let _ = inner::PubSubSubInner;

    // `std` is also an external library
    use std::collections::HashMap;
    let _ = HashMap::<i32, i32>::new();

    // using nested paths and glob
    #[allow(unused)]
    use std::io::{self, Write, prelude::*};
}

mod inner {
    pub mod sub_inner {
        pub mod sub_sub_inner {
            pub fn func() {
                println!("func");
            }

            pub struct SubInner;
        }

        pub struct SubInner;
    }

    // re-exporting
    pub use sub_inner::sub_sub_inner::SubInner as PubSubSubInner;
}

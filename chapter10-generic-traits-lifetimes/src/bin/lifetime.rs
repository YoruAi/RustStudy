fn main() {
    println!("lifetime");
    println!("borrow checker checks scope and lifetime tag");
    println!("lifetime annotations don’t change lifetime");
    println!("they tell how generic lifetime of multiple references relate to each other");
    println!("`'static` means the lifetime is as long as the whole procedure");

    let string1 = String::from("'a");
    let string2 = String::from("'a");
    let result;
    {
        let string3 = String::from("'b");
        result = longest(string1.as_str(), string2.as_str()); // compiler will disallow string3
    }
    println!("The longest string is {}", result);

    let i;
    {
        let string3 = String::from("'a");
        i = ImportantExcerpt {
            part: string1.as_str(), // compiler will disallow string3
        };
    }
    println!("part: {}", i.part);
}

// the returned reference will be valid as long as both of the parameters are valid
// `'a` is the short lifetime of the two parameters here
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// an instance of ImportantExcerpt can’t outlive the reference it holds in its part field
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl<'a> ImportantExcerpt<'a> {
    fn announce<'b>(&self, announcement: &'b str) -> &'b str {
        println!("Attention please: {announcement}");
        announcement
    }
}

// [lifetime elision rules]
// input lifetimes and output lifetimes
// 1. each parameter that is a reference gets its own lifetime parameter;
// 2. if there is exactly one input lifetime parameter,
//    that lifetime is assigned to all output lifetime parameters;
// 3. if there are multiple input lifetime parameters,
//    but one of them is &self or &mut self, the lifetime of self is assigned to all output.

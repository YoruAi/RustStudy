fn main() {
    println!(
        "trait object(must ptr, like &dyn Trait or Box<dyn Trait>) points to \
        both an instance of a type and a vtable used to look up trait methods"
    );
    println!(
        "duck typing: concerned only with the messages a value responds to \
        rather than the value’s concrete type"
    );
    println!(
        "dyn compatibility: some methods can't be stored in vtable, \
        such as functions that involve generic, Self and Sized, \
        see more rules at https://doc.rust-lang.org/reference/items/traits.html#dyn-compatibility"
    );

    println!("unlike trait object, using generics and trait bounds is monomorphized");
    println!("the code that results from monomorphization is doing static dispatch");
    println!("dynamic dispatch: the compiler emits code at runtime will know which method to call");
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    // trait object
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

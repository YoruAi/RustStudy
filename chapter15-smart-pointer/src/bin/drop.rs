fn main() {
    let c = CustomSmartPointer {
        data: String::from("a"),
    };
    let d = CustomSmartPointer {
        data: String::from("b"),
    };
    drop(d);
    println!("CustomSmartPointers created.");
}

struct CustomSmartPointer {
    data: String,
}
impl Drop for CustomSmartPointer {
    // can't be called explicitly
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

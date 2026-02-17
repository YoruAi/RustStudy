fn main() {
    println!("Send marker trait indicates ownership can be transferred between threads");
    println!("Any type composed entirely of Send types is automatically marked as Send");
    println!("Rc<T> doesn't impl Send for performance, so we should use Arc<T>");

    println!("Sync marker trait indicates it's safe to be referenced from multiple threads");
    println!("in other words, any type T implements Sync if &T implements Send");
    println!("RefCell<T>/Cell<T> doesn't impl Sync, so we should use Mutex<T>");

    println!("implementing Send and Sync manually is unsafe");
    println!("Send and Sync will be automatically marked");
}

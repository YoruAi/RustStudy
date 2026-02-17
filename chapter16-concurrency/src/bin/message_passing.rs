use std::sync::mpsc;
use std::thread;

fn main() {
    println!("by channel, data is sent from one thread to another.");
    println!("transmitter, receiver");

    // multiple producer, single consumer
    let (tx, rx) = mpsc::channel(); // auto infer
    let tx1 = tx.clone(); // clone more producer
    thread::spawn(move || {
        let val = String::from("hi");
        // send a value(move)
        tx.send(val).unwrap();
    });
    thread::spawn(move || {
        let val = String::from("hi1");
        // send a value(move)
        tx1.send(val).unwrap();
    });
    // blocking and recv a value, RecvError when channel is closed
    let received = rx.recv().unwrap();
    println!("Got: {received}");
    // or use for loop
    for received in &rx {
        println!("Got: {received}");
    }
    // non-blocking, TryRecvError when channel is empty
    let err = rx.try_recv().unwrap_err();
}

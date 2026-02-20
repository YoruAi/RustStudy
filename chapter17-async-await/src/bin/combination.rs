use std::thread;
use std::time::Duration;

fn main() {
    println!("the runtime's executor manages tasks, and tasks manage futures");
    println!(
        "If the work is very parallelizable (that is, CPU-bound), \
        such as processing a bunch of data where each part can be processed separately, \
        threads are a better choice."
    );
    println!(
        "If the work is very concurrent (that is, I/O-bound), \
        such as handling messages from different sources at different intervals or rates, \
        async is a better choice."
    );

    let (tx, mut rx) = trpl::channel();
    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    trpl::run(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}

use std::time::Duration;

fn main() {
    // block_on
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        trpl::join(fut1, fut2).await; // fair scheduling
    });

    trpl::run(async {
        // async channel
        let (tx, mut rx) = trpl::channel();
        // the order in which await keywords appear is also the order in which they’re executed
        // use move to drop the tx variable after send done
        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        let rx_fut = async {
            // while-let: loop if-let, until match failed
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    });
}

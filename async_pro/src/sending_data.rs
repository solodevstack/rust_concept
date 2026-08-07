use std::time::Duration;

//we used a for loop to process all the items received from a synchronous
//channel. Rust doesn’t yet have a way to use a for loop with an asynchronously produced series
//of items, however, so we need to use a loop we haven’t seen before: the while let
//conditional loop.

pub fn send_data() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let val = String::from("hi");
        tx.send(val).unwrap();
        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    })
}
pub fn sendmore_data() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(1000)).await;
        }
        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    })
}

//Right now, the async block where we send the messages only borrows tx because sending a
//message doesn’t require ownership, but if we could move tx into that async block, it would be
//dropped once that block ends.
pub fn sendmore_better_flow() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

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
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    })
}

pub fn sendmultiple_better_flow() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];
            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };
        
        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };
        //Finally, we switch from trpl::join to trpl::join! to handle the additional future: the join!
        trpl::join!(tx1_fut, tx_fut, rx_fut);
    })
}

pub fn linearlysend_data() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();
        let tx_fut = async {
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
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };
        trpl::join(tx_fut, rx_fut).await;
    })
}

use std::time::Duration;

fn main() {
    // trpl::block_on(async {
    //     let handle = trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }

    //     // Дожидаемся завершения цикла из дополнительного потока.
    //     handle.await.unwrap();
    // });

    // trpl::block_on(async {
    //     let fut1 = async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     let fut2 = async {
    //         for i in 1..5 {
    //             println!("hi number {i} from the second task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     };

    //     // Метод join дожидается завершения обоих Future
    //     trpl::join(fut1, fut2).await;
    // });

    // Sending Data Between Two Tasks Using Message Passing

    // trpl::block_on(async {
    //     let (tx, mut rx) = trpl::channel();

    //     let val = String::from("hi");
    //     tx.send(val).unwrap();

    //     let received = rx.recv().await.unwrap();
    //     println!("received '{received}'");
    // });

    // Sending and receiving multiple messages over the async channel and
    // sleeping with an await between each message

    // trpl::block_on(async {
    //     let (tx, mut rx) = trpl::channel();

    //     let vals = vec![
    //         String::from("hi"),
    //         String::from("from"),
    //         String::from("the"),
    //         String::from("future"),
    //     ];

    //     for val in vals {
    //         tx.send(val).unwrap();
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }

    //     while let Some(value) = rx.recv().await {
    //         println!("received '{value}'");
    //     }
    // });

    // Code Within One Async Block Executes Linearly

    // Separating send and recv into their own async blocks and awaiting the
    // futures for those blocks

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
    });
}

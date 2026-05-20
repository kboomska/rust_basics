use std::{thread, time::Duration};

fn main() {
    // Calling the slow function to simulate slow operations

    // trpl::block_on(async {
    //     let a = async {
    //         println!("'a' started.");
    //         slow("a", 30);
    //         slow("a", 10);
    //         slow("a", 20);
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         println!("'a' finished.");
    //     };

    //     let b = async {
    //         println!("'b' started.");
    //         slow("b", 75);
    //         slow("b", 10);
    //         slow("b", 15);
    //         slow("b", 350);
    //         trpl::sleep(Duration::from_millis(50)).await;
    //         println!("'b' finished.");
    //     };

    //     trpl::select(a, b).await;
    // });

    // Using trpl::sleep to let operations switch off making progress

    // trpl::block_on(async {
    //     let one_ms = Duration::from_millis(1);

    //     let a = async {
    //         println!("'a' started.");
    //         slow("a", 30);
    //         trpl::sleep(one_ms).await;
    //         slow("a", 10);
    //         trpl::sleep(one_ms).await;
    //         slow("a", 20);
    //         trpl::sleep(one_ms).await;
    //         println!("'a' finished.");
    //     };

    //     let b = async {
    //         println!("'b' started.");
    //         slow("b", 75);
    //         trpl::sleep(one_ms).await;
    //         slow("b", 10);
    //         trpl::sleep(one_ms).await;
    //         slow("b", 15);
    //         trpl::sleep(one_ms).await;
    //         slow("b", 350);
    //         trpl::sleep(one_ms).await;
    //         println!("'b' finished.");
    //     };

    //     trpl::select(a, b).await;
    // });

    // Using yield_now to let operations switch off making progress

    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

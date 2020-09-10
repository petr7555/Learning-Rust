use std::{thread, time};
use futures::executor::block_on;

async fn sleep_async() {
    let five_secs = time::Duration::from_secs(5);
    thread::sleep(five_secs);
}

async fn async_main() {
    println!("Begin sleeping");
    sleep_async().await;
    println!("Sleeping done")
}

fn main() {
    block_on(async_main());
}


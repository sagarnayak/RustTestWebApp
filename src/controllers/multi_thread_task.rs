use std::time::Duration;

use rand::Rng;
use std::thread::sleep;

#[get("/startMultiThreading")]
pub async fn start_multi_thread() -> &'static str {
    for i in 0..1000 {
        tokio::spawn(
            async move {
                // println!("Start timer {}.", &i);
                sleep(Duration::from_secs(rand::thread_rng().gen_range(1..3)));
                println!("Completed task number {}", i);
            }
        );
    }

    "started the multi thread"
}
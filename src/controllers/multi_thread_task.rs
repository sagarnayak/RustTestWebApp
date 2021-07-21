use std::thread::sleep;
use std::time::Duration;

use rand::Rng;

#[get("/startMultiThreading")]
pub fn start_multi_thread() -> &'static str {
    for i in 0..5 {
        tokio::spawn(
            async move {
                sleep(Duration::from_secs(rand::thread_rng().gen_range(1..5)));
                println!("Completed task number {}", i);
            }
        );
    }
    "started the multi thread"
}
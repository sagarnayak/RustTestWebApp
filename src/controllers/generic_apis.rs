use rand::Rng;
use std::thread::sleep;
use std::time::Duration;

#[get("/")]
pub fn index() -> &'static str {
    "Hello!"
}

#[get("/world")]
pub fn world() -> &'static str {
    "hello, world!"
}

#[get("/mayGet")]
pub fn may_get() -> Result<String, String> {
    let result_to_return = rand::thread_rng().gen_bool(0.5);
    if result_to_return {
        Ok("result ...".to_string())
    } else {
        Err("error...".to_string())
    }
}

#[get("/blockingTask")]
pub async fn blocking_task() -> Result<String, String> {
    sleep(Duration::from_secs(rand::thread_rng().gen_range(2..6)));
    Ok("Done .".to_string())
}

#[get("/me/<name>")]
pub fn me(name: &str) -> String {
    let result_to_return = format!("hi, {}", name);
    result_to_return
}
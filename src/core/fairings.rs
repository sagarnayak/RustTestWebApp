use rocket::{Data, Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

#[derive(Default, Clone)]
pub struct CounterFairing;

#[rocket::async_trait]
impl Fairing for CounterFairing {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response,
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        println!("got a req on {}", request.uri());
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        println!("got a  req from :: {:?}", request.client_ip());
        println!("sending response :: {:?}", response.status());
    }
}
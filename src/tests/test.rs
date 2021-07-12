extern crate rocket;


mod test_case_set_one {
    use rocket::http::Status;

    use crate::core::rocket_master::rocket;

    use super::rocket;

    #[test]
    #[should_panic]
    fn test_one() {
        panic!("oh no!");
    }

    #[test]
    fn test_two() {
        assert_eq!(2, 1 + 1);
    }

    #[test]
    fn rocket_test() {
        use rocket::local::blocking::Client;
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello!".into()));
    }

    #[rocket::async_test]
    async fn rocket_test_async() {
        use rocket::local::asynchronous::Client;
        let client = Client::tracked(rocket()).await.unwrap();
        let req = client.get("/blockingTask");

        let (r1, r2) = rocket::tokio::join!(req.clone().dispatch(), req.dispatch());
        assert_eq!(r1.status(), r2.status());
        assert_eq!(r1.status(), Status::Ok);

        let (s1, s2) = (r1.into_string().await, r2.into_string().await);
        assert_eq!(s1, s2);
        assert_eq!(s1.unwrap(), "Done .");
    }
}
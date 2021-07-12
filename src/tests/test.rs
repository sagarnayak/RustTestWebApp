extern crate rocket;


mod test_case_set_one {
    use rocket::{Build, Rocket};
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    use crate::controllers::error_handlers::not_found;
    use crate::controllers::routes;
    use crate::database::database_master;

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

    fn rocket() -> Rocket<Build> {
        rocket::build()
            .register("/", catchers![not_found])
            .mount("/", routes::get_routes())
            .manage(database_master::get_db_pools())
    }

    #[test]
    fn rocket_test() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/").dispatch();

        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string(), Some("Hello!".into()));
    }
}
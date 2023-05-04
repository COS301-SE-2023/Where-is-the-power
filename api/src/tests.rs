use super::build_rocket;
use rocket::http::Status;
use rocket::local::blocking::Client;
use rocket::uri;

#[test]
fn hello_world() {
    let client = Client::tracked(build_rocket()).expect("valid rocket instance");
    let response = client.get(format!("/hello{}", uri!(super::hi))).dispatch();
    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().unwrap(), "Hello World!");
}

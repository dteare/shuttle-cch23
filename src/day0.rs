use rocket::http::Status;
use rocket::{get, routes};

pub fn routes() -> Vec<rocket::Route> {
    routes![index, error]
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn error() -> Result<&'static str, Status> {
    Err(Status::InternalServerError)
}

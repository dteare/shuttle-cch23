use rocket::fs::{relative, NamedFile};
use rocket::{get, routes};
use std::path::{Path, PathBuf};

pub fn routes() -> Vec<rocket::Route> {
    routes![serve]
}
#[get("/assets/<path..>")]
pub async fn serve(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("public")).join(path);
    NamedFile::open(path).await.ok()
}

use chrono::Utc;
use rocket::http::Status;
use rocket::{get, post, routes, State};
use shuttle_persist::{PersistError, PersistInstance};

pub fn routes() -> Vec<rocket::Route> {
    routes![load, save]
}

pub struct Day12State {
    pub persist: PersistInstance,
}

#[post("/save/<packet_id>")]
pub async fn save(packet_id: String, state: &State<Day12State>) -> Result<(), Status> {
    let now = Utc::now();
    let now = now.timestamp().to_string();
    println!("@save {packet_id}={}", now);

    match state.persist.save(&packet_id, now.clone()) {
        Ok(_) => Ok(()),
        Err(PersistError::InvalidKey) => Err(Status::BadRequest),
        Err(e) => {
            println!("Error saving {packet_id} with timestamp {now}: {e}");
            Err(Status::InternalServerError)
        }
    }
}

#[get("/load/<packet_id>")]
pub async fn load(packet_id: String, state: &State<Day12State>) -> Result<String, Status> {
    match state.persist.load::<String>(&packet_id) {
        Ok(prev) => {
            let now = Utc::now();
            let now_as_seconds = now.timestamp();

            let prev: i64 = prev.parse().map_err(|e| {
                println!("Failed to parse previous timestamp {prev} for packet {packet_id}: {e}");
                Status::InternalServerError
            })?;
            let ago = now_as_seconds - prev;

            println!(
                "@load {packet_id} with timestamp {} was stored {} seconds ago",
                prev, ago
            );

            Ok(ago.to_string())
        }
        Err(PersistError::InvalidKey) => Err(Status::BadRequest),
        Err(e) => {
            println!("Error loading {packet_id}: {e}");
            Err(Status::InternalServerError)
        }
    }
}

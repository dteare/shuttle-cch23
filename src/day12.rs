use chrono::{DateTime, Datelike, TimeZone, Utc, Weekday};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::{get, post, routes, State};
use shuttle_persist::{PersistError, PersistInstance};
use std::collections::HashMap;
use ulid::Ulid;
use uuid::Uuid;

pub fn routes() -> Vec<rocket::Route> {
    routes![load, lsb, save, ulids]
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

#[post("/ulids", data = "<ulids>")]
async fn ulids(ulids: Json<Vec<String>>) -> Result<Json<Vec<String>>, Status> {
    println!("Received {} ULIDs", ulids.len());

    // Convert all the ULIDs to UUIDs
    let mut uuids = ulids
        .iter()
        .map(|ulid| {
            let ulid = Ulid::from_string(&ulid).map_err(|e| {
                println!("Failed to parse ULID {ulid}: {e}");
                Status::BadRequest
            })?;
            let uuid: Uuid = ulid.into();
            println!("Created uuid w/ version {}", uuid.get_version_num());
            Ok(uuid.to_string())
        })
        .collect::<Result<Vec<String>, Status>>()?;
    uuids.reverse();
    Ok(Json(uuids))
}

#[post("/ulids/<on_weekday>", data = "<ulids>")]
async fn lsb(
    on_weekday: u8,
    ulids: Json<Vec<String>>,
) -> Result<Json<HashMap<String, usize>>, Status> {
    let on_weekday = u32_to_weekday(on_weekday)?;
    println!(
        "@lsb Received {} ULIDs for weekday {}",
        ulids.len(),
        on_weekday
    );

    let uuids = ulids
        .iter()
        .map(|ulid| {
            let ulid = match Ulid::from_string(&ulid) {
                Ok(ulid) => ulid,
                Err(e) => {
                    println!("Failed to parse ULID {ulid}: {e}");
                    return Err(Status::BadRequest);
                }
            };

            let value: u128 = ulid.0;

            // Shift right by 80 bits to get the topmost 48 bits
            let timestamp_ms: u64 = (value >> 80) as u64;

            println!("Top 48 bits as u64: {}", timestamp_ms);

            let seconds = timestamp_ms / 1000;

            // Create a DateTime<Utc> object
            #[allow(deprecated)]
            let datetime: DateTime<Utc> = Utc.timestamp(seconds as i64, 0);
            Ok(datetime)
        })
        .collect::<Result<Vec<DateTime<Utc>>, Status>>()?;

    let on_christmas_eve = uuids
        .iter()
        .filter(|date_time| {
            let month = date_time.month();
            let day = date_time.day();
            month == 12 && day == 24
        })
        .count();

    let on_weekday = uuids
        .iter()
        .filter(|date_time| on_weekday == date_time.weekday())
        .count();

    let now = Utc::now();
    let in_future = uuids.iter().filter(|date_time| now < **date_time).count();

    let lsb_is_one = ulids
        .iter()
        .filter(|ulid| {
            // parse ulid string into ulid
            let ulid = match Ulid::from_string(&ulid) {
                Ok(ulid) => ulid,
                Err(e) => {
                    println!("Failed to parse ULID {ulid}: {e}");
                    return false;
                }
            };
            // take u128 out of the ulid and assign to value
            let value: u128 = ulid.0;
            value & 0x1 == 1
        })
        .count();

    let mut result = HashMap::new();
    result.insert("christmas eve".to_string(), on_christmas_eve);
    result.insert("weekday".to_string(), on_weekday);
    result.insert("in the future".to_string(), in_future);
    result.insert("LSB is 1".to_string(), lsb_is_one);

    Ok(Json(result))
}

// No idea why I can't use the chrono conversation directly but at this point fuck Santa and his stupid shit
fn u32_to_weekday(day_num: u8) -> Result<Weekday, Status> {
    match day_num {
        0 => Ok(Weekday::Mon),
        1 => Ok(Weekday::Tue),
        2 => Ok(Weekday::Wed),
        3 => Ok(Weekday::Thu),
        4 => Ok(Weekday::Fri),
        5 => Ok(Weekday::Sat),
        6 => Ok(Weekday::Sun),
        _ => Err(Status::BadRequest),
    }
}

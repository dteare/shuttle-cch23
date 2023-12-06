mod day0;
mod day1;
mod day4;
mod day6;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", day0::routes())
        .mount("/1", day1::routes())
        .mount("/4", day4::routes())
        .mount("/6", day6::routes());

    Ok(rocket.into())
}

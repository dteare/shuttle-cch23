mod day0;
mod day1;
mod day4;
mod day6;
mod day7;
mod day8;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build()
        .mount("/", day0::routes())
        .mount("/1", day1::routes())
        .mount("/4", day4::routes())
        .mount("/6", day6::routes())
        .mount("/7", day7::routes())
        .mount("/8", day8::routes());

    Ok(rocket.into())
}

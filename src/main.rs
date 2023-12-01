mod day0;
mod day1;

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", day0::routes()).mount("/1", day1::routes());

    Ok(rocket.into())
}

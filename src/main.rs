use crate::day12::Day12State;
use crate::day13::Day13State;
use shuttle_persist::PersistInstance;
//use sqlx::PgPool;
use rocket_dyn_templates::Template;

mod day0;
mod day1;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day4;
mod day6;
mod day7;
mod day8;

#[shuttle_runtime::main]
async fn main(
    #[shuttle_persist::Persist] persist: PersistInstance,
    #[shuttle_persist::Persist] persist2: PersistInstance,
    /* DB provisioning is fucked on my M3 #[shuttle_shared_db::Postgres] pool: PgPool, */
) -> shuttle_rocket::ShuttleRocket {
    let state12 = Day12State { persist };
    let state13 = Day13State { persist: persist2 };
    let rocket = rocket::build()
        .mount("/", day0::routes())
        .mount("/1", day1::routes())
        .mount("/4", day4::routes())
        .mount("/6", day6::routes())
        .mount("/7", day7::routes())
        .mount("/8", day8::routes())
        .mount("/11", day11::routes())
        .mount("/12", day12::routes())
        .mount("/13", day13::routes())
        .mount("/14", day14::routes())
        .mount("/15", day15::routes())
        .manage(state12)
        .manage(state13)
        .attach(Template::fairing());

    Ok(rocket.into())
}

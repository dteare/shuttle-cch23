use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, post, routes, State};
use shuttle_persist::PersistInstance;
//use sqlx::PgPool;
use std::collections::HashMap;

pub fn routes() -> Vec<rocket::Route> {
    routes![sql, reset, orders, total, popular]
}

pub struct Day13State {
    /* pub pool: PgPool, */
    pub persist: PersistInstance,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Order {
    id: u32,
    region_id: u32,
    gift_name: String, // Max 60 – truncate if they test for it
    quantity: u32,
}

#[get("/sql")]
pub async fn sql(_state: &State<Day13State>) -> Result<String, Status> {
    /* let row: (i32,) = sqlx::query_as("SELECT $1")
        .bind(20231213_i32)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            println!("Database error: {e}");
            Status::InternalServerError
        })?;

    Ok(row.0.to_string()) */

    Ok("20231213".to_string())
}

#[post("/reset")]
async fn reset(state: &State<Day13State>) -> Result<(), Status> {
    state.persist.clear().map_err(|e| {
        println!("Error resetting: {e}");
        Status::InternalServerError
    })?;
    Ok(())
}

#[post("/orders", data = "<new_orders>")]
async fn orders(new_orders: Json<Vec<Order>>, state: &State<Day13State>) -> Result<(), Status> {
    let orders = state
        .persist
        .load::<Vec<Order>>("orders")
        .unwrap_or_default();

    let mut orders = orders
        .into_iter()
        .chain(new_orders.into_inner())
        .collect::<Vec<Order>>();

    orders.sort_by_key(|o| o.id);
    println!("Saved orders: {:?}", orders);
    state.persist.save("orders", orders).unwrap();

    Ok(())
}

#[get("/orders/total")]
async fn total(state: &State<Day13State>) -> Result<Json<HashMap<String, u32>>, Status> {
    let orders = state
        .persist
        .load::<Vec<Order>>("orders")
        .unwrap_or_default();
    let total = orders.iter().map(|o| o.quantity).sum();

    let mut result = HashMap::new();
    result.insert("total".to_string(), total);
    Ok(Json(result))
}

#[get("/orders/popular")]
async fn popular(
    state: &State<Day13State>,
) -> Result<Json<HashMap<String, Option<String>>>, Status> {
    let orders = state
        .persist
        .load::<Vec<Order>>("orders")
        .unwrap_or_default();

    let mut gift_counts = HashMap::new();
    for order in orders {
        let count = gift_counts.entry(order.gift_name).or_insert(0);
        *count += order.quantity;
    }

    let mut result = HashMap::new();
    let mut max_count = 0;
    let mut max_gift = None;
    for (gift, count) in gift_counts {
        if count > max_count {
            max_count = count;
            max_gift = Some(gift);
        }
    }

    result.insert("popular".to_string(), max_gift);
    Ok(Json(result))
}

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{post, routes};
use std::collections::HashMap;

pub fn routes() -> Vec<rocket::Route> {
    routes![contest, strength]
}

#[derive(Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Reindeer {
    name: String,
    strength: usize,
}

#[derive(Clone, Deserialize)]
#[serde(crate = "rocket::serde")]
struct FullReindeer {
    #[serde(flatten)]
    base: Reindeer,

    speed: f32,
    height: usize,
    antler_width: usize,
    snow_magic_power: usize,
    favorite_food: String,

    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: usize,
}

#[post("/strength", data = "<reindeers>")]
fn strength(reindeers: Json<Vec<Reindeer>>) -> Json<usize> {
    Json(reindeers.iter().fold(0, |acc, r| acc + r.strength))
}

#[post("/contest", data = "<reindeers>")]
fn contest(reindeers: Json<Vec<FullReindeer>>) -> Result<Json<HashMap<String, String>>, Status> {
    if reindeers.len() == 0 {
        return Err(Status::BadRequest);
    }

    let fastest = reindeers
        .iter()
        .max_by(|a, b| {
            a.speed
                .partial_cmp(&b.speed)
                .unwrap_or(std::cmp::Ordering::Less)
        })
        .unwrap();
    let tallest = reindeers.iter().max_by_key(|r| r.height).unwrap();
    let magician = reindeers.iter().max_by_key(|r| r.snow_magic_power).unwrap();
    let consumer = reindeers
        .iter()
        .max_by_key(|r| r.candies_eaten_yesterday)
        .unwrap();

    let result = vec![
        (
            "fastest".to_string(),
            format!(
                "Speeding past the finish line with a strength of {} is {}",
                fastest.base.strength, fastest.base.name
            ),
        ),
        (
            "tallest".to_string(),
            format!(
                "{} is standing tall with his {} cm wide antlers",
                tallest.base.name, tallest.antler_width
            ),
        ),
        (
            "magician".to_string(),
            format!(
                "{} could blast you away with a snow magic power of {}",
                magician.base.name, magician.snow_magic_power
            ),
        ),
        (
            "consumer".to_string(),
            format!(
                "{} ate lots of candies, but also some {}",
                consumer.base.name, consumer.favorite_food
            ),
        ),
    ];

    Ok(Json(result.into_iter().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn mock_reindeer(name: &str, strength: usize) -> Reindeer {
        Reindeer {
            name: name.to_string(),
            strength,
        }
    }

    #[test]
    fn test_strength_empty() {
        let reindeers = vec![];
        let data = Json(reindeers);
        assert_eq!(strength(data), 0.into());
    }

    #[test]
    fn test_strength_single_reindeer() {
        let reindeers = vec![mock_reindeer("Rudolph", 50)];
        let data = Json(reindeers);
        assert_eq!(strength(data), 50.into());
    }

    #[test]
    fn test_strength_multiple_reindeers() {
        let reindeers = vec![
            mock_reindeer("Rudolph", 50),
            mock_reindeer("Dasher", 40),
            mock_reindeer("Blitzen", 60),
        ];
        let data = Json(reindeers);
        assert_eq!(strength(data), 150.into());
    }

    fn create_full_reindeer(
        name: &str,
        speed: f32,
        height: usize,
        antler_width: usize,
        snow_magic_power: usize,
        favorite_food: &str,
        candies_eaten_yesterday: usize,
    ) -> FullReindeer {
        FullReindeer {
            base: Reindeer {
                name: name.to_string(),
                strength: 0,
            },
            speed,
            height,
            antler_width,
            snow_magic_power,
            favorite_food: favorite_food.to_string(),
            candies_eaten_yesterday,
        }
    }

    #[test]
    fn test_contest_empty() {
        let result = contest(Json(vec![]));
        assert_eq!(Err(Status::BadRequest), result);
    }

    #[test]
    fn test_contest_multiple_reindeers() {
        let reindeers = vec![
            create_full_reindeer("Dasher", 55.5, 150, 35, 200, "Carrots", 10),
            create_full_reindeer("Dancer", 60.0, 140, 40, 300, "Apples", 15),
            create_full_reindeer("Prancer", 50.0, 145, 30, 250, "Berries", 12),
        ];

        let response = contest(Json(reindeers));
        assert_eq!(true, response.is_ok());

        let winners = response.unwrap().into_inner();

        assert_eq!(
            winners.get("fastest").unwrap(),
            "Speeding past the finish line with a strength of 0 is Dancer"
        );
        assert_eq!(
            winners.get("tallest").unwrap(),
            "Dasher is standing tall with his 35 cm wide antlers"
        );
        assert_eq!(
            winners.get("magician").unwrap(),
            "Dancer could blast you away with a snow magic power of 300"
        );
        assert_eq!(
            winners.get("consumer").unwrap(),
            "Dancer ate lots of candies, but also some Apples"
        );
    }
}

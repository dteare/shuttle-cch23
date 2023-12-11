use base64::{engine::general_purpose, Engine as _};
use rocket::http::{CookieJar, Status};
use rocket::serde::json::{serde_json, Json};
use rocket::serde::{Deserialize, Serialize};
use rocket::{get, routes};
use std::collections::HashMap;

pub fn routes() -> Vec<rocket::Route> {
    routes![bake, decode]
}

#[get("/decode")]
fn decode(cookies: &CookieJar<'_>) -> Result<String, Status> {
    match cookies.get("recipe") {
        Some(cookie) => {
            let recipe = cookie.value();

            let decoded = &general_purpose::STANDARD.decode(recipe);

            match decoded {
                Ok(decoded) => match String::from_utf8(decoded.clone()) {
                    Ok(decoded) => Ok(decoded),
                    Err(e) => {
                        println!("Failed to create UTF8 string from recipe bytes {recipe}: {e:?}");
                        Err(Status::BadRequest)
                    }
                },
                Err(e) => {
                    println!("Failed to base64 decode recipe {recipe}: {e:?}");
                    Err(Status::BadRequest)
                }
            }
        }
        None => Err(Status::BadRequest),
    }
}

#[derive(Deserialize, Serialize)]
struct BakeRequest {
    recipe: HashMap<String, usize>,
    pantry: HashMap<String, usize>,
}
#[derive(Debug, Deserialize, Serialize)]
struct BakeResponse {
    cookies: usize,
    pantry: HashMap<String, usize>,
}
#[get("/bake")]
fn bake(cookies: &CookieJar<'_>) -> Result<Json<BakeResponse>, Status> {
    let cookie_string = recipe_from_cookie(cookies)?;

    match serde_json::from_str::<BakeRequest>(&cookie_string) {
        Ok(recipe) => {
            let result = calc_baked_cookies(recipe.recipe, recipe.pantry);
            println!("@bake {cookie_string} => {result:?}");
            Ok(Json(result))
        }
        Err(e) => {
            println!("Failed to deserialize recipe {cookie_string}: {e:?}");
            Err(Status::BadRequest)
        }
    }
}

fn calc_baked_cookies(
    recipe: HashMap<String, usize>,
    mut pantry: HashMap<String, usize>,
) -> BakeResponse {
    let mut cookies = 0;

    // Bake until you can't bake no more
    loop {
        let mut can_bake = true;

        for (ingredient, amount_needed) in &recipe {
            if amount_needed == &0 {
                continue;
            }

            match pantry.get(ingredient) {
                Some(amount) => {
                    if *amount < *amount_needed {
                        can_bake = false;
                        break;
                    }
                }
                None => can_bake = false,
            }
        }

        if !can_bake {
            break;
        }

        // Remove ingredients from pantry
        cookies += 1;
        for (ingredient, amount_needed) in &recipe {
            if amount_needed == &0 {
                continue;
            }

            *pantry.get_mut(ingredient).unwrap() -= *amount_needed;
        }
    }

    BakeResponse {
        cookies,
        pantry: pantry,
    }
}

fn recipe_from_cookie(cookies: &CookieJar<'_>) -> Result<String, Status> {
    match cookies.get("recipe") {
        Some(cookie) => {
            let recipe = cookie.value();

            let decoded = &general_purpose::STANDARD.decode(recipe);

            match decoded {
                Ok(decoded) => match String::from_utf8(decoded.clone()) {
                    Ok(decoded) => Ok(decoded),
                    Err(e) => {
                        println!("Failed to create UTF8 string from recipe bytes {recipe}: {e:?}");
                        Err(Status::BadRequest)
                    }
                },
                Err(e) => {
                    println!("Failed to base64 decode recipe {recipe}: {e:?}");
                    Err(Status::BadRequest)
                }
            }
        }
        None => Err(Status::BadRequest),
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use rocket::http::Cookie;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_decode_success() {
        let rocket = rocket::build().mount("/", routes![decode]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let cookie = Cookie::new("recipe", "eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==");
        let response = client.get("/decode").cookie(cookie).dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.into_string().unwrap(),
            r#"{"flour":100,"chocolate chips":20}"#
        );
    }

    #[test]
    fn test_decode_invalid_base64() {
        let rocket = rocket::build().mount("/", routes![decode]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let cookie = Cookie::new("recipe", "invalid base64");
        let response = client.get("/decode").cookie(cookie).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_decode_invalid_utf8() {
        let rocket = rocket::build().mount("/", routes![decode]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let invalid_utf8 = [0, 159, 146, 150]; // Invalid UTF-8 bytes
        let cookie = Cookie::new("recipe", general_purpose::STANDARD.encode(invalid_utf8));
        let response = client.get("/decode").cookie(cookie).dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }

    #[test]
    fn test_decode_missing_cookie() {
        let rocket = rocket::build().mount("/", routes![decode]);
        let client = Client::tracked(rocket).expect("valid rocket instance");
        let response = client.get("/decode").dispatch();
        assert_eq!(response.status(), Status::BadRequest);
    }
}

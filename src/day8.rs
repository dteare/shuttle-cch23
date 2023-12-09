use rocket::http::Status;
use rocket::serde::json::serde_json;
use rocket::serde::Serialize;
use rocket::{get, routes};
use serde::Deserialize;
use thiserror::Error;

pub fn routes() -> Vec<rocket::Route> {
    routes![weight]
}

#[derive(Error, Debug)]
enum PokemonApiError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("Invalid pokedex number")]
    InvalidPokedexNumber,

    #[error("Failed to parse JSON")]
    Serde(#[from] serde_json::Error),
}
#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    id: usize,
    name: String,
    height: usize,
    weight: usize,
}
#[get("/weight/<pokedex_number>")]
async fn weight(pokedex_number: usize) -> Result<String, Status> {
    match load_pokemon(pokedex_number).await {
        Ok(pokemon) => {
            let kilograms = pokemon.weight as f32 / 10.0;
            Ok(kilograms.to_string())
        }
        Err(PokemonApiError::InvalidPokedexNumber) => Err(Status::BadRequest),
        Err(PokemonApiError::Network(e)) => {
            println!("Failed to connect with Pokeapi: {e:?}");
            Err(Status::BadGateway)
        }
        Err(PokemonApiError::Serde(e)) => {
            println!("Failed to parse pokemon response from Pokeapi: {e:?}");
            Err(Status::BadGateway)
        }
    }
}

async fn load_pokemon(id: usize) -> Result<Pokemon, PokemonApiError> {
    // Use reqwest to fetch the pokemon from the pokeapi
    let url = format!("https://pokeapi.co/api/v2/pokemon/{}", id);
    let response = reqwest::get(url).await?;

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        return Err(PokemonApiError::InvalidPokedexNumber);
    }

    let pokemon: Pokemon = response.json().await?;
    Ok(pokemon)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    async fn test_load_pikaku() {
        let pikachu = load_pokemon(25).await.unwrap();
        assert_eq!(pikachu.name, "pikachu");
        assert_eq!(pikachu.height, 4);
        assert_eq!(pikachu.weight, 60);
        assert_eq!(pikachu.id, 25);
    }

    #[test]
    async fn test_bad_input() {
        let pikachu = load_pokemon(99999999999);
        assert_eq!(
            pikachu.err().unwrap(),
            PokemonApiError::InvalidPokedexNumber
        );
    }
}

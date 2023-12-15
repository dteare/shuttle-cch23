use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};
use rocket::{post, routes, Request, Response};
use std::io::Cursor;

pub fn routes() -> Vec<rocket::Route> {
    routes![nice, game]
}

#[derive(Debug, Deserialize, Serialize)]
struct PasswordReq {
    input: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct NaughtyOrNice {
    result: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

struct NiceResponse {
    status: Status,
    result: NaughtyOrNice,
}

impl<'r> Responder<'r, 'static> for NiceResponse {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let string = serde_json::to_string(&self.result).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(string.len(), Cursor::new(string))
            .status(self.status)
            .ok()
    }
}

#[post("/nice", data = "<input>")]
fn nice(input: Json<PasswordReq>) -> NiceResponse {
    match is_nice(&input.input) {
        true => NiceResponse {
            result: NaughtyOrNice {
                result: "nice".to_string(),
                reason: None,
            },
            status: Status::Ok,
        },
        false => NiceResponse {
            result: NaughtyOrNice {
                result: "naughty".to_string(),
                reason: None,
            },
            status: Status::BadRequest,
        },
    }
}

fn is_nice(input: &str) -> bool {
    //  Must contain at least three vowels (aeiouy) – not necessarily distinct
    let mut vowels = 0;
    for c in input.chars() {
        if c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u' || c == 'y' {
            vowels += 1;
        }
    }

    println!("Vowels for {}: {:?}", input, vowels);
    if vowels < 3 {
        return false;
    }

    // at least one letter that appears twice in a row,
    let mut last_char = ' ';
    let mut letter_repeats = false;
    for c in input.chars() {
        if c.is_alphabetic() && c == last_char {
            letter_repeats = true;
            break;
        }
        last_char = c;
    }

    if !letter_repeats {
        return false;
    }

    // must not contain the substrings: ab, cd, pq, or xy.
    if input.contains("ab") || input.contains("cd") || input.contains("pq") || input.contains("xy")
    {
        return false;
    }

    true
}

// Custom Responder
struct GameResponse<T: Serialize> {
    status: Status,
    data: T,
}

impl<T: Serialize> GameResponse<T> {
    // Helper function to create a new GameResponse
    pub fn new(status: Status, data: T) -> Self {
        GameResponse { status, data }
    }
}
impl<'r, T: Serialize> Responder<'r, 'static> for GameResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let string = serde_json::to_string(&self.data).unwrap();
        Response::build()
            .header(ContentType::JSON)
            .sized_body(string.len(), Cursor::new(string))
            .status(self.status)
            .ok()
    }
}

enum NiceStringError {
    MustHaveAtLeast8Characters,
    MustContainUpperCaseEtc,
    MustContain5Digits,
    AllIntegersMustAddUpTo2023,
    MustContainJoy,
    MustRepeatLetter,
    MustContainUnicode,
    MustContainEmoji,
    Sha256MustEndWithA,
}
#[post("/game", data = "<input>")]
fn game(input: Json<PasswordReq>) -> GameResponse<NaughtyOrNice> {
    match nice_string_error(&input.input) {
        Some(error) => match error {
            NiceStringError::MustHaveAtLeast8Characters => GameResponse::new(
                Status::BadRequest,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("8 chars".to_string()),
                },
            ),
            NiceStringError::MustContainUpperCaseEtc => GameResponse::new(
                Status::BadRequest,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("more types of chars".to_string()),
                },
            ),
            NiceStringError::MustContain5Digits => GameResponse::new(
                Status::BadRequest,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("55555".to_string()),
                },
            ),
            NiceStringError::AllIntegersMustAddUpTo2023 => GameResponse::new(
                Status::BadRequest,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("math is hard".to_string()),
                },
            ),
            NiceStringError::MustContainJoy => GameResponse::new(
                Status::NotAcceptable,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("not joyful enough".to_string()),
                },
            ),
            NiceStringError::MustRepeatLetter => GameResponse::new(
                Status::UnavailableForLegalReasons,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("illegal: no sandwich".to_string()),
                },
            ),
            NiceStringError::MustContainUnicode => GameResponse::new(
                Status::RangeNotSatisfiable,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("outranged".to_string()),
                },
            ),
            NiceStringError::MustContainEmoji => GameResponse::new(
                Status::UpgradeRequired,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("😳".to_string()),
                },
            ),
            NiceStringError::Sha256MustEndWithA => GameResponse::new(
                Status::ImATeapot,
                NaughtyOrNice {
                    result: "naughty".to_string(),
                    reason: Some("not a coffee brewer".to_string()),
                },
            ),
        },
        None => GameResponse::new(
            Status::Ok,
            NaughtyOrNice {
                result: "nice".to_string(),
                reason: Some("that's a nice password".to_string()),
            },
        ),
    }
}

// Check the nice string criteria in order and return the first one that fails. If all pass, return AllPassed.
fn nice_string_error(input: &str) -> Option<NiceStringError> {
    // must be at least 8 characters long
    if input.len() < 8 {
        return Some(NiceStringError::MustHaveAtLeast8Characters);
    }

    //  must contain uppercase letters, lowercase letters, and digits
    if !contains_uppercase_lowercase_and_digits(input) {
        return Some(NiceStringError::MustContainUpperCaseEtc);
    }

    // must contain at least 5 digits
    if !contains_at_least_5_digits(input) {
        return Some(NiceStringError::MustContain5Digits);
    }

    // all integers (sequences of consecutive digits) in the string must add up to 2023
    if !all_integers_add_up_to_2023(input) {
        return Some(NiceStringError::AllIntegersMustAddUpTo2023);
    }

    // must contain the letters j, o, and y in that order and in no other order
    if !contains_joy(input) {
        return Some(NiceStringError::MustContainJoy);
    }

    // must contain a letter that repeats with exactly one other letter between them (like xyx)
    println!("Testing {} for REPEATS", input);
    if !contains_repeating_letter_with_one_between(input) {
        println!("MATCHED!!!!");
        return Some(NiceStringError::MustRepeatLetter);
    }

    // must contain at least one unicode character in the range [U+2980, U+2BFF]
    if !contains_unicode(input) {
        return Some(NiceStringError::MustContainUnicode);
    }

    // must contain at least one emoji
    if !contains_emoji(input) {
        return Some(NiceStringError::MustContainEmoji);
    }

    //  the hexadecimal representation of the sha256 hash of the string must end with an a
    if !sha256_ends_with_a(input) {
        return Some(NiceStringError::Sha256MustEndWithA);
    }

    None
}

fn sha256_ends_with_a(input: &str) -> bool {
    use sha2::{Digest, Sha256};

    let mut hasher = Sha256::new();
    let data = input.as_bytes();
    hasher.update(data);
    let hash = hasher.finalize();
    println!("Binary hash: {:?}", hash);

    let hex = hex::encode(hash);

    println!("Hex hash: {:?}", hex);
    hex.ends_with('a')
}

fn contains_emoji(input: &str) -> bool {
    for c in input.chars() {
        // Checking across multiple ranges where emojis are located
        if (c >= '\u{1F600}' && c <= '\u{1F64F}') || // Emoticons
            (c >= '\u{1F300}' && c <= '\u{1F5FF}') || // Misc Symbols and Pictographs
            (c >= '\u{1F680}' && c <= '\u{1F6FF}') || // Transport and Map Symbols
            (c >= '\u{1F900}' && c <= '\u{1F9FF}')
        {
            // Supplemental Symbols and Pictographs
            return true;
        }
    }
    false
}

fn contains_unicode(input: &str) -> bool {
    for c in input.chars() {
        if c >= '\u{2980}' && c <= '\u{2BFF}' {
            return true;
        }
    }
    false
}
fn contains_repeating_letter_with_one_between(s: &str) -> bool {
    if s.len() < 3 {
        return false;
    }

    let chars: Vec<char> = s.chars().collect();
    for i in 0..chars.len() - 2 {
        // There must be an OTHER character between them. So 2000 is not valid.
        if chars[i].is_alphabetic() && chars[i] == chars[i + 2] && chars[i] != chars[i + 1] {
            return true;
        }
    }
    false
}

fn contains_joy(input: &str) -> bool {
    // Must contain "joy" in that order and in no other order
    // strip every character that isn't "j", "o", or "y"
    let mut stripped = String::new();
    for c in input.chars() {
        if c == 'j' || c == 'o' || c == 'y' {
            stripped.push(c);
        }
    }

    stripped == "joy".to_string()
}

fn all_integers_add_up_to_2023(input: &str) -> bool {
    let mut sum = 0;
    let mut current_number = 0;
    for c in input.chars() {
        if c.is_ascii_digit() {
            current_number = current_number * 10 + c.to_digit(10).unwrap();
        } else {
            sum += current_number;
            current_number = 0;
        }
    }
    sum += current_number;
    sum == 2023
}

fn contains_uppercase_lowercase_and_digits(input: &str) -> bool {
    let mut contains_uppercase = false;
    let mut contains_lowercase = false;
    let mut contains_digits = false;
    for c in input.chars() {
        if c.is_ascii_uppercase() {
            contains_uppercase = true;
        }
        if c.is_ascii_lowercase() {
            contains_lowercase = true;
        }
        if c.is_ascii_digit() {
            contains_digits = true;
        }
    }
    contains_uppercase && contains_lowercase && contains_digits
}

fn contains_at_least_5_digits(input: &str) -> bool {
    let mut digits = 0;
    for c in input.chars() {
        if c.is_ascii_digit() {
            digits += 1;
        }
    }
    digits >= 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_pattern() {
        assert!(contains_repeating_letter_with_one_between(
            "example xyx string"
        ));
    }

    #[test]
    fn test_without_pattern() {
        assert!(!contains_repeating_letter_with_one_between("hello world"));
    }

    #[test]
    fn test_empty_string() {
        assert!(!contains_repeating_letter_with_one_between(""));
    }

    #[test]
    fn test_pattern_at_boundaries() {
        assert!(contains_repeating_letter_with_one_between("axa example"));
        assert!(contains_repeating_letter_with_one_between("example zbz"));
    }

    #[test]
    fn test_single_character_string() {
        assert!(!contains_repeating_letter_with_one_between("a"));
    }

    #[test]
    fn test_long_string() {
        let long_string = "a".repeat(10000) + "bxb" + &"a".repeat(10000);
        assert!(contains_repeating_letter_with_one_between(&long_string));
    }

    #[test]
    fn test_given_example() {
        assert!(!contains_repeating_letter_with_one_between(
            "23jPassword2000y"
        ));
    }

    #[test]
    fn test_is_nice() {
        assert!(is_nice("hello there"));
        assert!(!is_nice("he77o there"));
    }

    #[test]
    fn test_contains_joy() {
        assert!(contains_joy("joy"));
        assert!(contains_joy("2000.23.A joy joy"));
    }

    #[test]
    fn test_unicode() {
        assert_eq!(true, contains_emoji("🤔"));
    }
}

use rocket::serde::json::Json;
use rocket::{post, routes};
use std::collections::HashMap;

pub fn routes() -> Vec<rocket::Route> {
    routes![elf_count]
}

#[post("/", data = "<raw>")]
fn elf_count(raw: &str) -> Json<HashMap<String, usize>> {
    let elf_count = raw.matches("elf").count();
    let elf_on_a_shelf_count = count_overlapping_occurrences(raw, "elf on a shelf");
    let shelf_count = raw.matches("shelf").count();

    let mut result = HashMap::new();
    result.insert("elf".to_string(), elf_count);
    result.insert("elf on a shelf".to_string(), elf_on_a_shelf_count);
    result.insert(
        "shelf with no elf on it".to_string(),
        shelf_count - elf_on_a_shelf_count,
    );

    println!("@elf_count: {raw} => {:?}", result);
    Json(result)
}

fn count_overlapping_occurrences(haystack: &str, needle: &str) -> usize {
    let mut count = 0;
    let needle_len = needle.len();

    // Iterate over each possible starting position in `haystack`
    for start in 0..=haystack.len() {
        // Check if the remaining string is long enough to contain `needle`
        if start + needle_len <= haystack.len() {
            // If the substring starting at `start` matches `needle`, increment `count`
            if &haystack[start..start + needle_len] == needle {
                count += 1;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_elf() {
        let input = "This string does not contain the magic word.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &0);
    }

    #[test]
    fn test_single_elf() {
        let input = "An elf walked into the room.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &1);
    }

    #[test]
    fn test_multiple_elves() {
        let input = "The elf saw another elf.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &2);
    }

    #[test]
    fn test_no_spaces_required() {
        let input = "elfelfelf.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &3);
    }

    #[test]
    fn test_elf_in_other_words() {
        let input = "The shelf had an elfin figurine.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &2); // "shelf" and "elfin"
    }

    #[test]
    fn test_empty_string() {
        let input = "";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &0);
    }

    #[test]
    fn test_only_elf() {
        let input = "elf";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf").unwrap(), &1);
    }

    #[test]
    fn test_elf_on_a_shelf() {
        let input = "An elf on a shelf and another elf on a shelf were talking.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf on a shelf").unwrap(), &2);
    }

    #[test]
    fn test_shelf_with_no_elf_on_it() {
        let input = "There is a shelf with no elf on it, and another shelf over there.";
        let response = elf_count(input);
        assert_eq!(response.0.get("shelf with no elf on it").unwrap(), &2);
    }

    #[test]
    fn test_elf_on_a_shelf_and_not_on_a_shelf() {
        let input = "An elf on a shelf and another elf on a shelf were talking. Furthermore, there is a shelf with no elf on it, and another shelf over there.";
        let response = elf_count(input);
        assert_eq!(response.0.get("elf on a shelf").unwrap(), &2);
        assert_eq!(response.0.get("shelf with no elf on it").unwrap(), &2);
    }
}

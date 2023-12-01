use std::ffi::OsStr;
use std::path::PathBuf;
use rocket::{get, routes};
use rocket::http::Status;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/-1/error")]
fn error() -> Result<&'static str, Status> {
    Err(Status::InternalServerError)
}

#[get("/1/<path..>")]
fn calculate(path: PathBuf) -> Result<String, Status> {
    let mut acc = 0isize;
    let mut count = 0;

    for el in path.into_iter() {
        count = count + 1;
        if count > 20 {
            return Err(Status::BadRequest);
        }

        match parse_os_str_to_i32(el) {
            Ok(i) => {
                acc = acc ^ i
            },
            Err(_) => {
                return Err(Status::BadRequest)
            }
        }
    }
    let result = acc.pow(3);
    Ok(result.to_string())
}

#[shuttle_runtime::main]
async fn main() -> shuttle_rocket::ShuttleRocket {
    let rocket = rocket::build().mount("/", routes![index, error, calculate]);

    Ok(rocket.into())
}


fn parse_os_str_to_i32(os_str: &OsStr) -> Result<isize, &'static str> {
    let str_slice = os_str.to_str().ok_or("String conversion failed")?;
    str_slice.parse::<isize>().map_err(|_| "Parse to isize failed")
}


#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use std::path::PathBuf;

    // Helper function to create PathBuf from a string slice
    fn create_path_buf(path: &str) -> PathBuf {
        PathBuf::from(path)
    }

    #[test]
    fn test_calculate_success() {
        let path = create_path_buf("10");
        let result = calculate(path).unwrap();
        // Calculate the expected result: (10)**3 = 1000
        assert_eq!(result, "1000".to_string());
    }


    #[test]
    fn test_calculate_success_2() {
        let path = create_path_buf("4/5/8/10");
        let result = calculate(path).unwrap();
        // Calculate the expected result: 4^5^8^10
        assert_eq!(result, "27".to_string());
    }

    #[test]
    fn test_20_packets_ok() {
        let path = create_path_buf("0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0");
        let result = calculate(path).unwrap();
        // Calculate the expected result: (10)**3 = 1000
        assert_eq!(result, "0".to_string());
    }

    #[test]
    fn test_21_packets_bad() {
        let path = create_path_buf("0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0");
        let result = calculate(path);
        // Calculate the expected result: (10)**3 = 1000
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Status::BadRequest);
    }

    #[test]
    fn test_overflow_bad() {
        let path = create_path_buf("18446744073709551616");
        let result = calculate(path);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Status::BadRequest);
    }

    #[test]
    fn test_calculate_failure() {
        let path = create_path_buf("1/2/abc");
        let result = calculate(path);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), Status::BadRequest);
    }
}

use rocket::http::Status;
use rocket::{get, routes};
use std::ffi::OsStr;
use std::path::PathBuf;

pub fn routes() -> Vec<rocket::Route> {
    routes![calculate]
}

#[get("/1/<path..>")]
fn calculate(path: PathBuf) -> Result<String, Status> {
    let mut acc: isize = 0;
    let mut count = 0;

    for el in path.into_iter() {
        count = count + 1;
        if count > 20 {
            return Err(Status::BadRequest);
        }

        match parse_os_str_to_i32(el) {
            Ok(i) => acc = acc ^ i,
            Err(_) => return Err(Status::BadRequest),
        }
    }
    let result = acc.pow(3);
    Ok(result.to_string())
}

fn parse_os_str_to_i32(os_str: &OsStr) -> Result<isize, &'static str> {
    let str_slice = os_str.to_str().ok_or("String conversion failed")?;
    str_slice
        .parse::<isize>()
        .map_err(|_| "Parse to isize failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use std::path::PathBuf;

    #[test]
    fn test_calculate_success() {
        let path = PathBuf::from("10");
        let result = calculate(path).unwrap();
        // Calculate the expected result: (10)**3 = 1000
        assert_eq!(result, "1000".to_string());
    }

    #[test]
    fn test_calculate_success_2() {
        let path = PathBuf::from("4/5/8/10");
        let result = calculate(path).unwrap();
        // Calculate the expected result: 4^5^8^10
        assert_eq!(result, "27".to_string());
    }

    #[test]
    fn test_20_packets_ok() {
        let path = PathBuf::from("0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0");
        let result = calculate(path).unwrap();
        // Calculate the expected result: (10)**3 = 1000
        assert_eq!(result, "0".to_string());
    }

    #[test]
    fn test_21_packets_bad() {
        let path = PathBuf::from("0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0/0");
        let result = calculate(path);
        // Calculate the expected result: (10)**3 = 1000
        assert_eq!(result, Err(Status::BadRequest));
    }

    #[test]
    fn test_overflow_bad() {
        let path = PathBuf::from("18446744073709551616");
        let result = calculate(path);
        assert_eq!(result, Err(Status::BadRequest));
    }

    #[test]
    fn test_calculate_failure() {
        let path = PathBuf::from("1/2/abc");
        let result = calculate(path);
        assert_eq!(result, Err(Status::BadRequest));
    }
}

use rocket::serde::json::Json;
use rocket::serde::Deserialize;
use rocket::{post, routes};
use rocket_dyn_templates::{context, Template};
use serde::Serialize;

pub fn routes() -> Vec<rocket::Route> {
    routes![safe_input_handling, unsafe_input_handling]
}

#[derive(Debug, Deserialize, Serialize)]
struct Input {
    content: String,
}
#[post("/unsafe", data = "<input>")]
fn unsafe_input_handling(input: Json<Input>) -> String {
    // Horrible, horrible. But I need the points!
    format!(
        r#"<html>
  <head>
    <title>CCH23 Day 14</title>
  </head>
  <body>
    {}
  </body>
</html>"#,
        input.content
    )
}

#[post("/safe", data = "<input>")]
fn safe_input_handling(input: Json<Input>) -> Template {
    Template::render("safe_santa", context! { content: &input.content })
}

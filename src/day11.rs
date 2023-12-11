use image::GenericImageView;
use image::Pixel;
use rocket::form::Form;
use rocket::fs::{relative, NamedFile, TempFile};
use rocket::http::Status;
use rocket::{get, post, routes, FromForm};
use std::path::{Path, PathBuf};
use tokio::io::AsyncReadExt;

pub fn routes() -> Vec<rocket::Route> {
    routes![red_pixels, serve]
}
#[get("/assets/<path..>")]
pub async fn serve(path: PathBuf) -> Option<NamedFile> {
    let path = Path::new(relative!("public")).join(path);
    NamedFile::open(path).await.ok()
}

#[derive(FromForm)]
pub struct BullMode<'r> {
    image: TempFile<'r>,
}

#[post("/red_pixels", data = "<form>")]
pub async fn red_pixels(form: Form<BullMode<'_>>) -> Result<String, Status> {
    let file = form.into_inner().image;
    let mut file_data = Vec::new();
    file.open()
        .await
        .map_err(|e| {
            println!("Failed to open file: {e}");
            Status::InternalServerError
        })?
        .read_to_end(&mut file_data)
        .await
        .map_err(|e| {
            println!("Failed to read file contents: {e}");
            Status::InternalServerError
        })?;

    let img = image::load_from_memory(&file_data).map_err(|e| {
        println!("Error loading image from memory: {e}");
        Status::BadRequest
    })?;

    let mut magical_red = 0;
    for (_x, _y, pixel) in img.pixels() {
        let rgb = pixel.to_rgb();
        let (r, g, b) = (rgb[0], rgb[1], rgb[2]);

        if r as u32 > g as u32 + b as u32 {
            magical_red += 1;
        }
    }

    Ok(magical_red.to_string())
}

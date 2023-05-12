#[macro_use]
extern crate rocket;

use std::io::Read;
use std::path::Path;

use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile, relative, TempFile};
use rocket::serde::{json::Json, Serialize};

use crate::validation::ValidationType;

mod validation;

#[get("/")]
async fn index() -> NamedFile {
    let file_path = Path::new("static/index.html");
    NamedFile::open(file_path).await.unwrap()
}

#[non_exhaustive]
#[derive(FromFormField, Clone)]
enum PlainValidationType {
    Plain,
    #[field(value = "json")]
    WithJson,
    #[field(value = "cbor")]
    WithCbor,
}

#[derive(FromForm)]
struct Validation<'r> {
    #[field(name = "withExtra")]
    with_extra: PlainValidationType,
    cddl: &'r str,
    json: &'r str,
    file: TempFile<'r>,
}

fn get_temp_file_content(file: &TempFile) -> Vec<u8> {
    match file {
        TempFile::Buffered { content } => content.as_bytes().to_vec(),
        TempFile::File { path, len, .. } => {
            let mut file = std::fs::File::open(path).unwrap();
            let mut bytes = Vec::with_capacity(*len as usize);
            file.read_to_end(&mut bytes).unwrap();
            bytes
        }
    }
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ValidationResponse {
    title: String,
    message: String,
}

#[post("/validate", data = "<validation_data>")]
fn validate(validation_data: Form<Validation<'_>>) -> Json<Vec<ValidationResponse>> {
    let form_cddl = validation_data.cddl.to_string();
    let validation_type = match validation_data.with_extra {
        PlainValidationType::Plain => ValidationType::Plain(form_cddl),
        PlainValidationType::WithJson => {
            ValidationType::WithJson(form_cddl, validation_data.json.to_string())
        }
        PlainValidationType::WithCbor => ValidationType::WithCbor(
            form_cddl,
            get_temp_file_content(&validation_data.file),
        ),
    };

    Json(validation::validate_all(validation_type).iter()
        .map(|data| ValidationResponse {
            title: format!("{}:", data.0.clone()),
            message: data.1.clone(),
        })
        .collect())
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, validate])
        .mount("/static", FileServer::from(relative!("static")))
}

#[macro_use]
extern crate rocket;

use std::io::Read;
use std::path::Path;

use cddl_codegen::cli::Cli;
use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket::fs::{FileServer, NamedFile, relative, TempFile};
use rocket::serde::{json::Json, Serialize};
use tempfile::tempdir;

use codegen::{GEN_ZIP_FILE, generate_code};

use crate::validation::{ValidationLibrary, ValidationType};

mod codegen;
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
    cddl: &'r str,
    lib: ValidationLibrary,
    #[field(name = "withExtra")]
    with_extra: PlainValidationType,
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
    #[serde(rename = "alertType")]
    alert_type: String,
    title: String,
    message: Option<String>,
}

#[post("/validate", data = "<validation_data>")]
fn validate(validation_data: Form<Validation<'_>>) -> Json<ValidationResponse> {
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

    let result = validation::validate(validation_data.lib.clone(), validation_type);

    if result.is_ok() {
        return Json(ValidationResponse {
            alert_type: "success".to_string(),
            title: "Validation successful".to_string(),
            message: None,
        });
    }

    Json(ValidationResponse {
        alert_type: "warning".to_string(),
        title: "Validation failed".to_string(),
        message: Some(result.err().unwrap()),
    })
}

#[post("/generate", data = "<cddl>")]
async fn generate(cddl: String) -> Option<NamedFile> {
    let root = tempdir().unwrap();
    let mut args = Cli::default();
    generate_code(root.path(), &cddl, &mut args).unwrap();
    NamedFile::open(root.path().join(GEN_ZIP_FILE).as_path())
        .await
        .ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, validate, generate])
        .mount("/static", FileServer::from(relative!("static")))
}

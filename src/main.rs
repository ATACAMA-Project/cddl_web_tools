mod validation;

#[macro_use]
extern crate rocket;

extern crate rocket_dyn_templates;

use crate::validation::{ValidationLibrary, ValidationType};
use rocket::form::Form;
use rocket::fs::{relative, FileServer, TempFile};
use rocket::Request;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};
use std::io::Read;

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[non_exhaustive]
#[derive(FromFormField, Clone)]
enum PlainValidationType {
    #[field(value = "")]
    Plain,
    #[field(value = "json")]
    WithJson,
    #[field(value = "cbor")]
    WithCbor,
}

#[derive(FromForm)]
struct Validation<'r> {
    cddl: &'r str,
    crateT: ValidationLibrary,
    vtype: PlainValidationType,
    extra: &'r str,
    file: TempFile<'r>,
}

fn get_temp_file_content(file: &TempFile) -> Result<Vec<u8>, std::io::Error> {
    match file {
        TempFile::Buffered { content } => Ok(content.as_bytes().to_vec()),
        TempFile::File { path, len, .. } => {
            let mut file = std::fs::File::open(path.as_ref().left().unwrap())?;
            let mut bytes = Vec::with_capacity(*len as usize);
            file.read_to_end(&mut bytes)?;
            Ok(bytes)
        }
    }
}

#[post("/validate", data = "<validation_data>")]
fn validate(validation_data: Form<Validation<'_>>) -> Template {
    let form_cddl = validation_data.cddl.to_string();
    let validation_type = match validation_data.vtype {
        PlainValidationType::Plain => ValidationType::Plain(form_cddl),
        PlainValidationType::WithJson => {
            ValidationType::WithJson(form_cddl, validation_data.extra.to_string())
        }
        PlainValidationType::WithCbor => ValidationType::WithCbor(
            form_cddl,
            get_temp_file_content(&validation_data.file).unwrap(),
        ),
    };

    let result = validation::validate(validation_data.crateT.clone(), validation_type);

    if result.is_ok() {
        return Template::render(
            "response",
            context! {
                mtype: "success",
                details: "The CDDL is valid!",
            },
        );
    }

    Template::render(
        "response",
        context! {
            mtype: "warning",
            details: result.err().unwrap(),
        },
    )
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/", routes![index, validate])
        .mount("/static", FileServer::from(relative!("static")))
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render(
        "error",
        context! {
            title: "404",
            details: format!("The following page was not found {}", req.uri()),
        },
    )
}

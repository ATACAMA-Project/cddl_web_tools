mod validation;

#[macro_use]
extern crate rocket;

extern crate rocket_dyn_templates;

use crate::validation::{ValidationLibrary, ValidationType};
use rocket::form::Form;
use rocket::fs::{relative, FileServer, TempFile};
use rocket::{Request, Response};
use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};
use std::io::Read;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;

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

fn get_temp_file_content(file: &TempFile) -> Vec<u8> {
    match file {
        TempFile::Buffered { content } => content.as_bytes().to_vec(),
        TempFile::File { path, len, .. } => {
            let mut file = std::fs::File::open(path).unwrap();
            let mut bytes = Vec::with_capacity(*len as usize);
            file.read_to_end(&mut bytes);
            bytes
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
            get_temp_file_content(&validation_data.file),
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
        .attach(CORS)
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

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
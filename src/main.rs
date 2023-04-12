mod validation;

#[macro_use]
extern crate rocket;

extern crate rocket_dyn_templates;

use rocket::Request;
use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket::fs::{FileServer, relative};
use crate::validation::{ValidationLibrary, ValidationType};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {})
}

#[derive(Debug, FromForm)]
struct Validation<'r> {
    cddl: &'r str,
}

#[post("/validate", data = "<validation_data>")]
fn validate(validation_data: Form<Validation<'_>>) -> Template {
    let result = validation::validate(
        ValidationLibrary::Cddl,
        ValidationType::Plain(validation_data.cddl.to_string()),
    );

    if result.is_ok() {
        return Template::render("response", context! {
            mtype: "success",
            details: "The CDDL is valid!",
        })
    }

    Template::render("response", context! {
        mtype: "danger",
        details: result.err().unwrap(),
    })
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
    Template::render("error", context! {
        title: "404",
        details: format!("The following page was not found {}", req.uri()),
    })
}

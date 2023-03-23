#[macro_use]
extern crate rocket;

extern crate rocket_dyn_templates;

use rocket::Request;
use rocket::{Build, Rocket};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use rocket::fs::{FileServer, relative};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        cddl: "",
    })
}

#[derive(Debug, FromForm)]
struct Validation<'r> {
    cddl: &'r str,
}

#[post("/", data = "<validation_data>")]
fn validate(validation_data: Option<Form<Validation<'_>>>) -> Template {
    match validation_data {
        None => index(),
        Some(data) => {
            Template::render("index", context! {
                cddl: data.cddl,
            })
        }
    }
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
    Template::render("error",context! {
        title: "404",
        details: format!("The following page was not found {}", req.uri()),
    })
}

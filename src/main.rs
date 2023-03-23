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
        name: "1",
    })
}

#[derive(Debug, FromForm)]
struct Validation<'r> {
    cddl: &'r str,
}

#[post("/validate", data = "<validation_data>")]
fn validate(validation_data: Option<Form<Validation<'_>>>) -> Template {
    println!("Task: {:?}", validation_data);
    Template::render("index", context! {
        name: "2",
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
    Template::render("error",context! {
        title: "404",
        details: format!("The following page was not found {}", req.uri()),
    })
}

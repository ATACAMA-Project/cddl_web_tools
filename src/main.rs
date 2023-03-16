#[macro_use]
extern crate rocket;

extern crate rocket_dyn_templates;

use rocket::Request;
use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
    Template::render("index", context! {
        name: "1",
    })
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index])
        .attach(Template::fairing())
        .register("/", catchers![not_found])
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    Template::render("error/404",context! {
        path: req.uri().path().to_string(),
    })
}

#![feature(mutex_unlock)]

pub mod codegen;

#[macro_use]
extern crate rocket;

use cddl_codegen::cli::Cli;
use codegen::generate_code;
use codegen::GEN_ZIP_FILE;
use rocket::fs::NamedFile;
use rocket::{Build, Rocket};
use tempfile::tempdir;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/generate", data = "<cddl>")]
async fn generate(cddl: String) -> Option<NamedFile> {
    let root = tempdir().unwrap();
    generate_code(root.path(), &cddl, None).unwrap();
    NamedFile::open(root.path().join(GEN_ZIP_FILE).as_path())
        .await
        .ok()
}

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build().mount("/", routes![index, generate])
}

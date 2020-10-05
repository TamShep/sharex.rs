#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate rocket_multipart_form_data;
extern crate serde_json;

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    MultipartFormData, MultipartFormDataField, MultipartFormDataOptions,
};
struct UploadData {
    file: Vec<u8>,
    token: String,
}

#[post("/", data = "<data>")]
fn uploadhandler(contentT: &ContentType, data: Data) -> String {
    if *contentT == ContentType::FormData {
        let mut options = MultipartFormDataOptions::new();

        options
            .allowed_fields
            .push(MultipartFormDataField::file("file"));

        options
            .allowed_fields
            .push(MultipartFormDataField::text("token"));

        let parsed = MultipartFormData::parse(contentT, data, options).unwrap();

        let token = parsed.files.get(&"token".to_string());

        format!("Token: {:?}\n", token);
    }
    format!("Error: request is not a multipart formdata")
}

#[get("/")]
fn index() -> &'static str {
    "rustsharex is running."
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, uploadhandler])
        .launch();
}

use std::fmt::format;
use std::{iter::Product, sync::RwLock};

use rocket::data::ToByteUnit;
use rocket::http::ext::IntoCollection;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::serde_json;
use rocket::Data;
use rocket::{serde::json::Json, State};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[macro_use]
extern crate rocket;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
struct StoreItem {
    #[serde(default)]
    id: i32,
    name: String,
    description: String,
    #[serde(default)]
    img: Vec<u8>,
}

struct StoreState {
    last_id: RwLock<i32>,
    products: RwLock<Vec<StoreItem>>,
}

#[post("/product", format = "json", data = "<item>")]
fn new_item(item: Json<StoreItem>, state: &State<StoreState>) -> String {
    let mut product = state.products.write().unwrap();
    let mut last_id = state.last_id.write().unwrap();
    let mut parsed = item.0;
    parsed.id = *last_id;
    *last_id += 1;
    println!("Got {:?}", parsed.clone());
    product.push(parsed);
    "Ok".to_owned()
}

#[get("/product/<id>")]
fn get_product(id: i32, state: &State<StoreState>) -> status::Custom<String> {
    let products = state.products.write().unwrap();
    let result = products.iter().find(|x| x.id == id);
    match result {
        Some(product) => status::Custom(Status::Accepted, serde_json::to_string(product).unwrap()),
        None => status::Custom(Status::BadRequest, "Not found".to_owned()),
    }
}

#[get("/products")]
fn get_products(state: &State<StoreState>) -> String {
    let products = state.products.write().unwrap();
    serde_json::to_string(&*products).unwrap()
}

#[delete("/product/<id>")]
fn delete_product(id: i32, state: &State<StoreState>) -> String {
    let mut products = state.products.write().unwrap();
    let old_size = products.len();
    *products = products
        .iter()
        .filter(|product| product.id != id)
        .cloned()
        .collect();
    let new_size = products.len();
    format!("{} products affected", old_size - new_size)
}

#[put("/product/<id>", data = "<updated_fields>")]
fn update_product(id: i32, updated_fields: String, state: &State<StoreState>) -> String {
    let mut products = state.products.write().unwrap();
    let result = products.iter_mut().find(|x| x.id == id).unwrap();

    let value: Value = serde_json::from_str(&updated_fields).unwrap();
    if let Some(new_name) = value["name"].as_str() {
        result.name = new_name.to_owned();
    }
    if let Some(new_description) = value["description"].as_str() {
        result.description = new_description.to_owned();
    }
    "Ok".to_string()
}

#[post("/product/<id>/image", data = "<icon>")]
async fn update_img(id: i32, icon: Vec<u8>, state: &State<StoreState>) -> std::io::Result<String> {
    let mut products = state.products.write().unwrap();
    if let Some(result) = products.iter_mut().find(|x| x.id == id) {
        // let mut res = Vec::<u8>::new();
        // let res = img.open(512.kibibytes()).into_bytes().await?;
        // result.img = res.into_inner();
        result.img = icon;
        Ok(format!("Got img size={}", result.img.len()))
    } else {
        Ok("Index not found".to_owned())
    }
}

#[get("/product/<id>/image")]
fn get_img(id: i32, state: &State<StoreState>) -> Vec<u8> {
    let products = state.products.write().unwrap();
    if let Some(result) = products.iter().find(|x| x.id == id) {
        result.img.clone()
    } else {
        vec![]
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                get_product,
                new_item,
                get_products,
                delete_product,
                update_product,
                update_img,
                get_img
            ],
        )
        .manage(StoreState {
            last_id: RwLock::new(0),
            products: RwLock::new(vec![]),
        })
}

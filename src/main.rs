#![allow(dead_code)]
use reqwest;
use serde::Deserialize;

fn main() {
    blocking_get().unwrap();
}

fn blocking_get() -> Result<(), Box<dyn std::error::Error>> {
    let api_url = "http://localhost:3000/users/";
    let response = reqwest::blocking::get(api_url)?; 

    let body = response.json::<Data>()?;
    println!("_id: {:?}", body._id);
    println!("nombre: {:?}", body._id);
    println!("email: {:?}", body._id);

    Ok(())
}

#[derive(Deserialize)]
struct Data {
    _id: String,
    email: String,
    nombre: String
}
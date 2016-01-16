#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::io;
use std::io::prelude::*;
use std::fs::File;

use error::{SortableError};

pub mod error;

#[derive(Serialize,Deserialize,Debug)]
struct Product {
    product_name: String,
    manufacturer: String,
    model: String,
    family: String,
    #[serde(rename="announced-date")]
    announced_date: String
}

fn read_products(products_path: &str) -> Result<Vec<Product>, SortableError> {
    let mut f = try!(File::open(products_path));
    let mut file_contents = String::from("["); // since the text file only contains lines of objects, but no array

    try!(f.read_to_string(&mut file_contents));

    file_contents.push_str("]");

    let all_products: Vec<Product> = try!(serde_json::from_str(&file_contents));
    Ok(all_products)
/*
    match all_products {
        Ok(ap) => Ok(ap),
        Err(ref e) => SortableError::JsonError(e)
    }*/
}

fn main() {
    println!("Hello, world!");
    
    let all_products = match read_products("../products.txt") {
        Ok(products) => products,
        Err(e) => panic!("Error reading products: {}", e)
    };

}

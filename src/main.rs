#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(serde_macros)]

extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;

use error::SortableError;

pub mod error;

#[derive(Serialize,Deserialize,Debug)]
struct Product {
    product_name: String,
    #[serde(skip_serializing)]
    manufacturer: String,
    #[serde(skip_serializing)]
    model: String,
    #[serde(skip_serializing)]
    family: String,
    #[serde(rename="announced-date",skip_serializing)]
    announced_date: String,
    listings: Vec<String>,
}

#[derive(Serialize,Deserialize,Debug)]
struct Listing {
    title: String,
    manufacturer: String,
    currency: String,
    price: String,
}

fn read_objects<T: serde::de::Deserialize>(file_path: &str) -> Result<Vec<T>, SortableError> {
    let mut f = try!(File::open(file_path));
    let mut file_contents = String::new();

    try!(f.read_to_string(&mut file_contents));

    // map each line in the file to a new object
    // TODO: work out how to handle errors here???
    let all_objects: Vec<T> = file_contents.lines()
                                           .map(|line| serde_json::from_str(&line))
                                           .filter(|o| o.is_ok())
                                           .map(|o| o.unwrap())
                                           .collect();
    Ok(all_objects)
}

fn write_objects<T: serde::ser::Serialize>(file_path: &str,
                                           all_objects: &[T])
                                           -> Result<(), SortableError> {
    let mut f = try!(File::create(file_path));

    // map each line in the file to a new object
    // TODO: work out how to handle errors here???
    for o in all_objects {
        try!(writeln!(f, "{}", serde_json::to_string(o).unwrap()));
    }
    Ok(())
}

fn main() {
    let mut all_products: Vec<Product> = match read_objects("../products.txt") {
        Ok(products) => products,
        Err(e) => panic!("Error reading products: {}", e),
    };

    // sort the products by manufacturer
    &mut all_products[..].sort_by(|p1, p2| p1.manufacturer.cmp(&p2.manufacturer));

    println!("Successfully read {} products", all_products.len());
    for p in all_products.iter().take(5) {
        println!("{:?}", p);
    }

    let all_listings: Vec<Listing> = match read_objects("../listings.txt") {
        Ok(listings) => listings,
        Err(e) => panic!("Error reading listings: {}", e),
    };

    println!("Successfully read {} listings", all_listings.len());
    for p in all_listings.iter().take(5) {
        println!("{:?}", p);
    }

    write_objects("../results.txt", &all_products[..]).unwrap();
}

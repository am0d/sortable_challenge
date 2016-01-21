#![feature(custom_derive, plugin, custom_attribute)]
#![plugin(clippy,serde_macros)]

extern crate serde;
extern crate serde_json;

use std::io::prelude::*;
use std::fs::File;
use std::collections::{BTreeMap, BTreeSet};

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
    listings: Vec<Listing>,
}

impl PartialEq for Product {
    fn eq(&self, other: &Self) -> bool {
        self.product_name == other.product_name
    }
}

impl PartialOrd for Product {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.product_name.cmp(&other.product_name))
    }
}

impl Ord for Product {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.product_name.cmp(&other.product_name)
    }
}
impl Eq for Product {}

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
    // Note: This just drops any line that has an error
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

    // Write each obect on a new line
    for o in all_objects {
        try!(writeln!(f, "{}", serde_json::to_string(o).unwrap()));
    }
    Ok(())
}

fn match_listings_to_products(all_products: &mut [Product], all_listings: Vec<Listing>) -> u32 {
    println!(" -> Preprocessing the products ...");
    let mut product_map = BTreeMap::new();
    for product in all_products.iter() {
        // pre-process the keywords
        for keyword in format!("{} {} {}",
                               product.manufacturer,
                               product.family,
                               product.model)
                           .split_whitespace()
                           .map(|s| String::from(s.to_lowercase())) {
            product_map.entry(keyword).or_insert(vec![]).push(product);
        }
    }
    println!(" -> Matching listings ...");
    let mut matched_products_count = 0u32;
    let mut word_count = BTreeMap::new();

    for listing in all_listings {
        let mut best_match: (Option<&mut Product>, usize) = (None, 0);
        let mut matched_products = BTreeSet::new();

        for listing_keyword in (listing.title)[..]
                                   .split(|c| c == ' ' || c == '/')
                                   .map(|s| s.to_lowercase()) {
            let products_for_keyword = product_map.get(&listing_keyword);

            let current_product_count = matched_products.len();

            matched_products = match products_for_keyword {
                None => matched_products,
                Some(products) if matched_products.is_empty() => {
                    for p in products.iter() {
                        matched_products.insert(p);
                    }
                    matched_products
                },
                Some(products) => {
                    let mut products_set = BTreeSet::new();
                    for p in products.iter() {
                        products_set.insert(p);
                    }
                    let subset = 
                        matched_products.intersection(&products_set)
                    ;
                    let mut matched_products = BTreeSet::new();
                    for p in subset {
                        matched_products.insert(*p);
                    }
                    if current_product_count > 0 && matched_products.is_empty() {
                        *word_count.entry(listing_keyword).or_insert(0) += 1;
                    }
                    matched_products
                }
            }
        }

        if matched_products.len() == 1 {
            for product in matched_products.into_iter() {
                //(*product).listings.push(listing);
                matched_products_count += 1;
            }
        }
    }

    println!("({}), {:?}", word_count.len(), word_count);

    matched_products_count
}

fn main() {
    let mut all_products: Vec<Product> = match read_objects("../products.txt") {
        Ok(products) => products,
        Err(e) => panic!("Error reading products: {}", e),
    };

    // sort the products by manufacturer
    // &mut all_products[..].sort_by(|p1, p2| p1.manufacturer.cmp(&p2.manufacturer));

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

    println!("Matching listings to products ...");
    let total_listings = all_listings.len();
    let matched_listings = match_listings_to_products(&mut all_products[..], all_listings);
    println!("\tMatched {} out of {} listings",
             matched_listings,
             total_listings);

    println!("Writing results to file ...");
    // We call unwrap() as a lazy way to show an error
    write_objects("../results.txt", &all_products[..]).unwrap();

    println!("Done");
}

use std::{collections::HashMap, error::Error, fs};
fn main() {
    println!("Hello, world!");
    read_data("./src/sample.txt".to_string());
}

struct Mapping {
    destination_start: i32,
    source_start: i32,
    range: i32,
}

struct BaseQuery {
    seeds: Vec<i32>,
    seed_soil_mapping: Vec<Mapping>,
    soil_fertilizer_mapping: Vec<Mapping>,
    fertilizer_water_mapping: Vec<Mapping>,
    water_light_mapping: Vec<Mapping>,
    light_temperature_mapping: Vec<Mapping>,
    temperature_humidity_mapping: Vec<Mapping>,
    humidity_location_mapping: Vec<Mapping>,
}

struct QualifiedMappings {
    seed_soil_mapping: HashMap<i32, i32>,
    soil_fertilizer_mapping: HashMap<i32, i32>,
    fertilizer_water_mapping: HashMap<i32, i32>,
    water_light_mapping: HashMap<i32, i32>,
    light_temperature_mapping: HashMap<i32, i32>,
    temperature_humidity_mapping: HashMap<i32, i32>,
    humidity_location_mapping: HashMap<i32, i32>,
}

fn read_data(path: String) -> Result<BaseQuery, String> {
    let input = fs::read_to_string(path).expect("Should have been able to read file.");

    let sections: Vec<&str> = input.split("\n").collect();

    Err("Invalid".to_string())
}

use std::fs::File;

use serde::{Deserialize, Serialize};
use serde_json::from_reader;

use crate::configdata::ConfigData;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize, Eq, Hash)]
pub struct Categories{
    categories: Vec<String>
}

impl Categories {
    pub fn new() -> Categories {
        let categories: Vec<String> = Vec::new();
        Categories { categories }
    }

    #[allow(unused)]
    pub fn display_all(&self) {

        print!("\n\nAvailable categories:\n");
        for c in &self.categories {
            print!("\t{}\n",c);
        }
        print!("\n\n");
    }

    pub fn init_categories() -> Categories {
        //open config file
        let config_file_path: String = format!("{}","files/config.json");
        let config_file: File = File::open(config_file_path).expect("ERROR - CONFIG FILE NOT FOUND");
        let config_data: ConfigData = from_reader(config_file).expect("ERROR - FAILED TO DESERIALIZE CONFIG FILE");

        //read categories file
        let categories_file_path: String = format!("{}",&config_data.CATEGORIES_FILE);
        let cache_file: File = File::open(categories_file_path.clone()).expect("ERROR - CACHE FILE NOT FOUND");
        let categories: Categories = from_reader(cache_file).expect("ERROR - FAILED TO DESERIALIZE CATEGORIES FILE");

        return categories;
    }

    pub fn get_categories(&self) -> &Vec<String> {
        return &self.categories;
    }

    // pub fn string_to_category(cat: &str) -> Categories {
    //     match cat {
    //         "Groceries" | "groceries" => Categories::Groceries,
    //         "Food" | "food" => Categories::Food,
    //         "Subscriptions" | "subscriptions" => Categories::Subscriptions,
    //         "Car" | "car" => Categories::Car,
    //         "House" | "house" => Categories::House,
    //         "Fun" | "fun" => Categories::Fun,
    //         _ => Categories::DEFAULT
    //     }
    // }

    // pub fn category_to_string(cat: Categories) -> String {
    //     match cat {
    //         Categories::Groceries => "Groceries".to_string(),
    //         Categories::Food => "Food".to_string(),
    //         Categories::Subscriptions => "Subscriptions".to_string(),
    //         Categories::Car => "Car".to_string(),
    //         Categories::House => "House".to_string(),
    //         Categories::Fun => "Fun".to_string(),
    //         _ => "DEFAULT".to_string()
    //     }
    // }
}

use std::{collections::HashMap, sync::Mutex};
use serde::{Deserialize, Serialize};
use crate::categories; //bring items from the module into the current scope

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryEntry {
    category: String,
    entries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JSONData {
    values: Vec<CategoryEntry>
}

impl JSONData{
    /*pub fn display_all(&self){
        for v in &self.values {
            println!("Category: {}", v.category);
            println!("Entries:");
            for e in &v.entries {
                println!("\t{}",e);
            }
            println!("----------------------------------------");
        }
    }*/
    
    #[allow(non_snake_case)]
    pub fn read_json_data(&self, data: &Mutex<HashMap<String,categories::Categories>>){
        let mut categs = data.lock().unwrap();
        for v in &self.values {
            for e in &v.entries {
                let cat: categories::Categories = categories::Categories::string_to_category(&v.category);
                categs.insert(e.to_string(), cat);
            }
        }
    }
}
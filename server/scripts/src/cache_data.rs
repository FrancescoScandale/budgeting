use std::{collections::HashMap, fs::File, io::Write, sync::Mutex};
use serde::{Deserialize, Serialize};
use crate::categories::Categories; //bring items from the module into the current scope

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryEntry {
    category: String,
    entries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CacheData {
    values: Vec<CategoryEntry>
}

impl CacheData{
    // pub fn display_all(&self){
    //     for v in &self.values {
    //         println!("Category: {}", v.category);
    //         println!("Entries:");
    //         for e in &v.entries {
    //             println!("\t{}",e);
    //         }
    //         println!("----------------------------------------");
    //     }
    // }
    
    pub fn read_json_data(&self, data: &Mutex<HashMap<String,Categories>>){
        let mut categs: std::sync::MutexGuard<'_, HashMap<String, Categories>> = data.lock().unwrap();
        for v in &self.values {
            for e in &v.entries {
                let cat: Categories = Categories::string_to_category(&v.category);
                categs.insert(e.to_string(), cat);
            }
        }
    }

    pub fn write_json_data(&mut self, data: &Mutex<HashMap<String,Categories>>, cache_file_path: String) {
        let categs: std::sync::MutexGuard<'_, HashMap<String, Categories>> = data.lock().unwrap();
        let mut found_category: bool;

        for desc in categs.keys() {
            found_category = false;
            for json_cat in &mut self.values {
                if categs.get(desc).unwrap().clone() == Categories::string_to_category(&json_cat.category) {
                    found_category = true;
                    if !json_cat.entries.contains(desc) {
                        json_cat.entries.push(desc.to_string());
                    }
                }
            }

            if !found_category {
                self.values.push(CategoryEntry {
                    category: Categories::category_to_string(categs.get(desc).unwrap().clone()).to_lowercase(),
                    entries: vec![desc.to_string()]
                });
            }
        }

        let json_string = serde_json::to_string_pretty(&self).unwrap();

        let mut cache_file: File = File::create(cache_file_path).expect("ERROR - FAILED TO OPEN CACHE FILE FOR FINAL WRITING");
        cache_file.write_all(json_string.as_bytes()).expect("ERROR - FAILED TO WRITE TO CACHE FILE");
    }
}
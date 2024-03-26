//------------------------------LIBRARIES------------------------------
use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Write}, str::FromStr, sync::Mutex};
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

//------------------------------MODULES------------------------------
mod categories; //imports the contents of categories.rs into the current module
mod cache_data;

//------------------------------GLOBAL VARIABLES------------------------------
lazy_static!{
    static ref CATEGORIZED_DATA: Mutex<HashMap<String,categories::Categories>> = Mutex::new(HashMap::new());
}

//------------------------------UTILITY AND STRUCTS------------------------------
// #[allow(non_snake_case)]
// fn display_all_CATEGORIES(){
//     let categories = CATEGORIZED_DATA.lock().unwrap();

//     println!("TESTING THE HASH MAP");
//     for(key, val) in categories.iter() {
//         println!("{} -> {}",categories::Categories::category_to_string(val.clone()),key);
//     }
// }

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Files {
    CACHE_FILE: String,
    REPORT_FILE: String
}

struct Entries {
    date: String,
    description: String,
    amount: f32,
    category: categories::Categories
}

impl Entries {
    fn display(entries: &Entries) {
        println!("{} - {} - {} - {}", entries.date, entries.amount, entries.description, categories::Categories::category_to_string(entries.category.clone()));
    }
}
//------------------------------FUNCTIONS------------------------------
fn find_category(desc: &str) -> categories::Categories {
    let categories: Vec<categories::Categories> = categories::Categories::get_categories();
    let mut inserted_category: String = String::new();
    let mut chosen_category: categories::Categories = categories::Categories::DEFAULT;
    let mut valid: bool = false;

    //lock mutex and check if description already present in cache
    let mut categs = CATEGORIZED_DATA.lock().unwrap();
    if categs.contains_key(desc) {
        println!("Description already present: {desc} -> {}",categories::Categories::category_to_string(categs.get(desc).unwrap().clone()));
        return categs.get(desc).unwrap().clone();
    }

    println!();
    print!("{}\nWhat is the category of the previous description? ",desc);
    io::stdout().flush().expect("Flush failed."); //needed to de-buffer the print
    while !valid {
        //user inserts input and it gets trimmed
        _ = io::stdin().read_line(&mut inserted_category);
        chosen_category = categories::Categories::string_to_category(inserted_category.trim());

        //check if valid input
        if categories.contains(&chosen_category) {
            valid = true;
        } else {
            print!("{} is not a valid category! Insert a valid one: ",inserted_category.trim());
            io::stdout().flush().expect("Flush failed."); //needed to de-buffer the print
            inserted_category.clear();
        }
    }

    //insert description and category
    categs.insert(desc.to_string(), chosen_category.clone());

    return chosen_category;
}
//------------------------------------------------------------------------
fn read_csv_input() -> io::Result<()> {
    categories::Categories::display_all();

    //read config file
    let config_file_path: &str = "files/config.json";
    let config_file: File = File::open(config_file_path).expect("ERROR - CONFIG FILE NOT FOUND");
    let config_data: Files = from_reader(config_file).expect("ERROR - FAILED TO DESERIALIZE CONFIG FILE");

    //read cache file
    let cache_file_path = config_data.CACHE_FILE.clone();
    let cache_file: File = File::open(cache_file_path).expect("ERROR - CACHE FILE NOT FOUND");
    let mut cache_data: cache_data::CacheData = from_reader(cache_file).expect("ERROR - FAILED TO DESERIALIZE CATEGORIES FILE");
    cache_data.read_json_data(&CATEGORIZED_DATA);

    //read report file
    let report_file_path = config_data.REPORT_FILE;
    let report_file: File = File::open(report_file_path).expect("ERROR - REPORT FILE NOT FOUND");
    let reader: BufReader<File> = io::BufReader::new(report_file);

    let mut entries: Vec<Entries> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line?;
        let mut v: Vec<&str> = line.split(";").collect();
        
        
        if v[5] == "" || v[5] == " " { //earnings
            v[5] = "0.0";
        } else {    //expenses (to be placed into categories)
            let amount: String = v[5].replace(",", ".");

            //clean description
            let tmp_desc_1: Vec<&str> = v[3].split("c/o:").collect();
            let desc_1: &str = match tmp_desc_1.len() {
                1 => tmp_desc_1[0],
                _ => tmp_desc_1[1]
            };
            let regex: Regex = Regex::new(r"IT\s*in data").unwrap();
            let tmp_desc_2: Vec<&str> = regex.split(desc_1).collect();
            let desc: &str = tmp_desc_2[0].trim();

            //establish a category
            let cat: categories::Categories = find_category(desc);

            //push in the structure
            entries.push(Entries { 
                date: v[2].to_string(), 
                description: desc.to_string(), 
                amount: f32::from_str(&amount).unwrap().abs(), 
                category: cat });
        }
    }

    //print final results
    println!("\n\n\n---------------------FINAL RESULT---------------------");
    for e in entries {
        Entries::display(&e);
    }
    cache_data.write_json_data(&CATEGORIZED_DATA, config_data.CACHE_FILE.clone());

    // display_all_CATEGORIES();

    Ok(())
}
//-------------------------------------------------------------------------
#[tokio::main]
async fn main() {
    _ = read_csv_input();
    
    //println!("EXECUTING RUST PROGRAM!");
}
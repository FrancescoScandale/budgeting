//------------------------------LIBRARIES------------------------------
use std::{collections::HashMap, env, fs::File, io::{self, BufRead, BufReader, Write}, path::PathBuf, str::FromStr, sync::Mutex};
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

//------------------------------MODULES------------------------------
mod categories; //imports the contents of categories.rs into the current module
mod cache_data;
mod entries;
mod display;

//------------------------------GLOBAL VARIABLES------------------------------
lazy_static!{
    static ref CATEGORIZED_DATA: Mutex<HashMap<String,categories::Categories>> = Mutex::new(HashMap::new());
}

//------------------------------UTILITY AND STRUCTS------------------------------
#[allow(non_snake_case, unused)]
fn display_all_CATEGORIES(){
    let categories = CATEGORIZED_DATA.lock().unwrap();

    println!("TESTING THE HASH MAP");
    for(key, val) in categories.iter() {
        println!("{} -> {}",categories::Categories::category_to_string(val.clone()),key);
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
struct Files {
    CACHE_FILE: String,
    REPORT_FILE: String
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
        //println!("Description already present: {desc} -> {}",categories::Categories::category_to_string(categs.get(desc).unwrap().clone()));
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
fn read_csv_input() -> Vec<entries::Entries> {
    // categories::Categories::display_all();
    let path: PathBuf = env::current_dir().unwrap(); 
    let mut path_string: String = path.to_str().unwrap().to_string();

    //read config file
    if !path_string.contains("scripts") {
        path_string = format!("{}{}",path_string,"/scripts"); //append scripts
    }
    path_string = format!("{}{}",path_string,"/");
    let config_file_path: String = format!("{}{}",path_string.as_str(),"files/config.json");
    let config_file: File = File::open(config_file_path).expect("ERROR - CONFIG FILE NOT FOUND");
    let config_data: Files = from_reader(config_file).expect("ERROR - FAILED TO DESERIALIZE CONFIG FILE");

    //read cache file
    let cache_file_path: String = format!("{}{}",path_string.as_str(),&config_data.CACHE_FILE);
    let cache_file: File = File::open(cache_file_path.clone()).expect("ERROR - CACHE FILE NOT FOUND");
    let mut cache_data: cache_data::CacheData = from_reader(cache_file).expect("ERROR - FAILED TO DESERIALIZE CATEGORIES FILE");
    cache_data.read_json_data(&CATEGORIZED_DATA);

    //read report file
    let report_file_path: String = format!("{}{}",path_string.as_str(),&config_data.REPORT_FILE);
    let report_file: File = File::open(report_file_path).expect("ERROR - REPORT FILE NOT FOUND");
    let reader: BufReader<File> = io::BufReader::new(report_file);

    let mut result: Vec<entries::Entries> = Vec::new();

    for line in reader.lines().skip(1) {
        let line = line.unwrap();
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
            result.push(entries::Entries::new(
                v[2].to_string(),
                desc.to_string(),
                f32::from_str(&amount).unwrap().abs(),
                cat));
        }
    }

    cache_data.write_json_data(&CATEGORIZED_DATA, cache_file_path);

    // display_all_CATEGORIES();

    return result;
}
//-------------------------------------------------------------------------
fn main() {
    // let path = env::current_dir().unwrap();
    // println!("The current directory is {}", path.display());

    let final_result: Vec<entries::Entries> = read_csv_input();

    // println!("\n");
    // entries::Entries::display_vector(&final_result);

    let json_string: String = serde_json::to_string(&final_result).unwrap();
    println!("{}", json_string);
}
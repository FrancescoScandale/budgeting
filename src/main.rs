use std::{collections::HashMap, fs::File, io::{self, BufRead, BufReader, Write}, str::FromStr, sync::Mutex};
use regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;


#[derive(Serialize, Deserialize, Debug)]
struct CategoryEntry {
    category: String,
    entries: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JSONData {
    values: Vec<CategoryEntry>
}

//definition of the static variable CATEGORIES, initialized as empty
lazy_static!{
    static ref CATEGORIES: Mutex<HashMap<String,Categories>> = Mutex::new(HashMap::new());
}

#[allow(non_snake_case)]
fn display_all_CATEGORIES(){
    let categories = CATEGORIES.lock().unwrap();

    println!("TESTING THE HASH MAP");
    for(key, val) in categories.iter() {
        println!("{} -> {}",Categories::category_to_string(val.clone()),key);
    }
}

#[derive(Debug, PartialEq, Clone)] // Derive the PartialEq trait for Categories
enum Categories{
    DEFAULT,
    Groceries,
    Food,
    Subscriptions,
    Car,
    House,
    Fun
}

impl JSONData{
    /*fn display_all(&self){
        for v in &self.values {
            println!("Category: {}", v.category);
            println!("Entries:");
            for e in &v.entries {
                println!("\t{}",e);
            }
            println!("----------------------------------------");
        }
    }*/

    fn read_json_data(&self){
        for v in &self.values {
            for e in &v.entries {
                let cat:Categories = Categories::string_to_category(&v.category);
                let mut categs = CATEGORIES.lock().unwrap();
                categs.insert(e.to_string(), cat);
            }
        }
    }
}

struct Entries {
    date: String,
    description: String,
    amount: f32,
    category: Categories
}

impl Entries {
    fn display(entries: &Entries) {
        println!("{} - {} - {} - {}", entries.date, entries.amount, entries.description, Categories::category_to_string(entries.category.clone()));
    }
}

impl Categories {
    fn display_all() {
        let categories: Vec<Categories> = Categories::get_categories();

        print!("\n\nAvailable categories:\n");
        for c in categories {
            print!("\t{}\n",Categories::category_to_string(c));
        }
        print!("\n\n");
    }

    fn get_categories() -> Vec<Categories> {
        let categories: Vec<Categories> = vec![
            Categories::Groceries,
            Categories::Food,
            Categories::Subscriptions,
            Categories::Car,
            Categories::House,
            Categories::Fun
        ];

        return categories;
    }

    fn string_to_category(cat: &str) -> Categories {
        match cat {
            "Groceries" | "groceries" => Categories::Groceries,
            "Food" | "food" => Categories::Food,
            "Subscription" | "subscription" => Categories::Subscriptions,
            "Car" | "car" => Categories::Car,
            "House" | "house" => Categories::House,
            "Fun" | "fun" => Categories::Fun,
            _ => Categories::DEFAULT
        }
    }

    fn category_to_string(cat: Categories) -> String {
        match cat {
            Categories::Groceries => "Groceries".to_string(),
            Categories::Food => "Food".to_string(),
            Categories::Subscriptions => "Subscriptions".to_string(),
            Categories::Car => "Car".to_string(),
            Categories::House => "House".to_string(),
            Categories::Fun => "Fun".to_string(),
            _ => "DEFAULT".to_string()
        }
    }
}

fn find_category(desc: &str) -> Categories {
    let categories: Vec<Categories> = Categories::get_categories();
    let mut inserted_category: String = String::new();
    let mut chosen_category: Categories = Categories::DEFAULT;
    let mut valid: bool = false;

    //lock mutex and check if description already present
    let mut categs = CATEGORIES.lock().unwrap();
    if categs.contains_key(desc) {
        println!("Description already present: {desc} -> {}",Categories::category_to_string(categs.get(desc).unwrap().clone()));
        return categs.get(desc).unwrap().clone();
    }

    println!();
    print!("{}\nWhat is the category of the previous description? ",desc);
    io::stdout().flush().expect("Flush failed."); //needed to de-buffer the print
    while !valid {
        //user inserts input and it gets trimmed
        _ = io::stdin().read_line(&mut inserted_category);
        chosen_category = Categories::string_to_category(inserted_category.trim());

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

fn read_csv_input() -> io::Result<()> {
    let categories_file_path = "files/categories.json";
    let categories_file: File = File::open(categories_file_path).expect("ERROR - CATEGORIES FILE NOT FOUND");
    let categories_data: JSONData = from_reader(categories_file).expect("ERROR - FAILED TO DESERIALIZE JSON");
    categories_data.read_json_data();

    let report_file_path = "files/report.csv";
    let report_file: File = File::open(report_file_path).expect("ERROR - REPORT FILE NOT FOUND");
    let reader: BufReader<File> = io::BufReader::new(report_file);

    let mut entries: Vec<Entries> = Vec::new();

    Categories::display_all();

    for line in reader.lines().skip(1) {
        let line = line?;
        let mut v: Vec<&str> = line.split(";").collect();
        
        //clean amount
        if v[5] == "" || v[5] == " " {
            v[5] = "0.0";
        }
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
        let cat: Categories = find_category(desc);

        //push in the structure
        entries.push(Entries { 
            date: v[2].to_string(), 
            description: desc.to_string(), 
            amount: f32::from_str(&amount).unwrap().abs(), 
            category: cat });
    }

    //print final results
    for e in entries {
        Entries::display(&e);
    }
    display_all_CATEGORIES();

    Ok(())
}

fn main() {
    _ = read_csv_input();
}
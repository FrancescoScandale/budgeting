use std::{fs::File, io::{self, BufRead, BufReader, Write}, str::FromStr};
use regex::Regex;
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

impl JSONData{
    fn display_all(values: Vec<CategoryEntry>){
        for v in values {
            println!("Category: {}", v.category);
            println!("Entries:");
            for e in v.entries {
                println!("\t{}",e);
            }
            println!("----------------------------------------");
        }
    }
}

#[derive(Debug, PartialEq)] // Derive the PartialEq trait for Categories
enum Categories{
    DEFAULT,
    Groceries,
    Food,
    Subscriptions,
    Car,
    House,
    Fun
}

struct Entries {
    date: String,
    description: String,
    amount: f32,
    category: Categories
}

impl Entries {
    fn display(entries: &Entries) {
        println!("{} - {} - {} - {}", entries.date, entries.amount, entries.description, Categories::category_to_string(&entries.category));
    }
}

//TODO: to avoid all these shenanigans, probably better to just set the enum to use &str instead of proper types
impl Categories {
    fn get_categories() -> Vec<Categories> {
        let mut categories: Vec<Categories> = Vec::new();

        categories.push(Categories::Groceries);
        categories.push(Categories::Food);
        categories.push(Categories::Subscriptions);
        categories.push(Categories::Car);
        categories.push(Categories::House);
        categories.push(Categories::Fun);

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

    fn category_to_string(cat: &Categories) -> String {
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

    fn display_all() {
        let categories: Vec<Categories> = Categories::get_categories();

        print!("\t\tAvailable categories:\n");
        for c in categories {
            print!("\t{}\n",Categories::category_to_string(&c));
        }
        print!("\n\n");
    }
}

fn find_category(desc: &str) -> Categories {
    let categories: Vec<Categories> = Categories::get_categories();
    let mut inserted_category: String = String::new();
    let mut chosen_category: Categories = Categories::DEFAULT;
    let mut valid: bool = false;

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

    return chosen_category;
}

fn read_csv_input() -> io::Result<()> {
    let categories_file_path = "files/categories.json";
    let categories_file: File = File::open(categories_file_path).expect("ERROR - CATEGORIES FILE NOT FOUND");
    let categories_data: JSONData = from_reader(categories_file).expect("ERROR - FAILED TO DESERIALIZE JSON");
    JSONData::display_all(categories_data.values);

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

    for e in entries {
        Entries::display(&e);
    }

    Ok(())
}

fn main() {
    _ = read_csv_input();
}
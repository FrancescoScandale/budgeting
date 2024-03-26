use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Categories{
    DEFAULT,
    Groceries,
    Food,
    Subscriptions,
    Car,
    House,
    Fun
}

impl Categories {
    #[allow(unused)]
    pub fn display_all() {
        let categories: Vec<Categories> = Categories::get_categories();

        print!("\n\nAvailable categories:\n");
        for c in categories {
            print!("\t{}\n",Categories::category_to_string(c));
        }
        print!("\n\n");
    }

    pub fn get_categories() -> Vec<Categories> {
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

    pub fn string_to_category(cat: &str) -> Categories {
        match cat {
            "Groceries" | "groceries" => Categories::Groceries,
            "Food" | "food" => Categories::Food,
            "Subscriptions" | "subscriptions" => Categories::Subscriptions,
            "Car" | "car" => Categories::Car,
            "House" | "house" => Categories::House,
            "Fun" | "fun" => Categories::Fun,
            _ => Categories::DEFAULT
        }
    }

    pub fn category_to_string(cat: Categories) -> String {
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

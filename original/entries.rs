use serde::{Deserialize, Serialize};

use crate::categories::Categories;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Entries {
    date: String,
    description: String,
    amount: f32,
    category: Categories
}

impl Entries {
    pub fn new(d: String, desc: String, a: f32, c: Categories) -> Entries {
        Entries {
            date: d,
            description: desc,
            amount: a,
            category: c
        }
    }

    pub fn get_category(&self) -> Categories { self.category }

    pub fn get_amount(&self) -> f32 { self.amount }

    #[allow(unused)]
    pub fn display_vector(entries: &Vec<Entries>) {
        for e in entries {
            println!("{} - {} - {} - {}", e.date, e.amount, e.description, Categories::category_to_string(e.category.clone()));
        }
    }
}
use std::collections::HashMap;

use crate::entries;
use crate::categories::Categories;

pub fn display_cli_result(final_result: Vec<entries::Entries>) {
    let totals: HashMap<Categories,f32> = compute_totals(final_result);

    let percentages: HashMap<Categories,f32> = compute_percentages(totals);

    let min_percentage: f32 = compute_min_percentage(percentages.clone());

    display_final_result(min_percentage,percentages);
}

fn compute_totals(final_result: Vec<entries::Entries>) -> HashMap<Categories,f32> {
    let mut totals: HashMap<Categories,f32> = HashMap::new();

    for fr in final_result {
        let c: Categories = fr.get_category();
        //insert category if not present
        if !totals.contains_key(&c) {
            totals.insert(c,0.0);
        }

        totals.insert(c, totals.get(&c).unwrap() + fr.get_amount());
    }

    //println!("{:?}", totals);

    return totals;
}

fn compute_percentages(totals: HashMap<Categories,f32>) -> HashMap<Categories,f32> {
    let total: f32 = totals.clone().into_values().sum();
    let mut percentages: HashMap<Categories, f32> = HashMap::new();

    for (c,a) in totals {
        percentages.insert(c, (a/total)*100.0);
    }

    return percentages;
}

fn compute_min_percentage(percentages: HashMap<Categories,f32>) -> f32 {
    let mut min_percentage: f32 = 100.0;

    //compute min percentage
    for p in percentages.into_values() {
        if p < min_percentage {
            min_percentage = p;
        }

    }
    
    //cap the min_percentage at 5%
    if min_percentage < 5.0 { min_percentage = 5.0; }

    return min_percentage;

}

fn display_final_result(min_percentage: f32, percentages: HashMap<Categories,f32>) {
    for (c,p) in percentages {
        let char_number: usize = (p/min_percentage).ceil() as usize; //usize needed for repeat()
        println!("{}: {:.2}%",Categories::category_to_string(c),p);
        println!("{}\n","=".repeat(char_number*3)); //displays characters for better visualization
    }
}
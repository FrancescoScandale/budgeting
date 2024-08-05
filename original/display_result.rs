use std::collections::HashMap;

use crate::entries;
use crate::categories::Categories;

pub fn display_cli_result(final_result: Vec<entries::Entries>, argument: &str) -> HashMap<String,String> {
    let totals: HashMap<Categories,f32> = compute_totals(final_result);

    let percentages: HashMap<Categories,f32> = compute_percentages(totals);

    let min_percentage: f32 = compute_min_percentage(percentages.clone());

    if argument == "cli"{
        display_result(min_percentage,percentages.clone());
        return HashMap::new();
    }

    //convert f32 to String to use it in the HTML visualization
    let mut html_percentages: HashMap<String,String> = HashMap::new();
    for (c,p) in percentages {
        html_percentages.insert(Categories::category_to_string(c), p.to_string());
    }

    return html_percentages;
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
        let p: f32 = (a/total)*100.0;
        let rounded_p: f32 = (p * 100.0).round() / 100.0;
        percentages.insert(c, rounded_p);
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

fn display_result(min_percentage: f32, percentages: HashMap<Categories,f32>) {
    for (c,p) in percentages {
        let char_number: usize = (p/min_percentage).ceil() as usize; //usize needed for repeat()
        println!("{}: {}%",Categories::category_to_string(c),p);
        println!("{}\n","=".repeat(char_number*3)); //displays characters for better visualization
    }
}
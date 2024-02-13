use std::fs::File;

use firebase_rs::*;
use serde::{Deserialize, Serialize};
use serde_json::from_reader;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize, Debug)]
struct ConfigData {
    DATABASE_URL: String
}

#[derive(Serialize, Deserialize, Debug)]
struct MyTestStructure {
  entries: Vec<String>
}

// pub async fn init_firestore() -> FirestoreDb {
//     //read config file
//     let config_file_path = "files/config.json";
//     let config_file: File = File::open(config_file_path).expect("ERROR - CONFIG FILE NOT FOUND");
//     let config_data: ConfigData = from_reader(config_file).expect("ERROR - FAILED TO DESERIALIZE CONFIG FILE");
    
//     return FirestoreDb::new(config_data.PROJECT_ID).await.expect("ERROR - FAILED TO INITIALISE FIRESTORE DB");
// }

pub fn init_firestore() -> Firebase {
    //read config file
    let config_file_path = "files/config.json";
    let config_file: File = File::open(config_file_path).expect("ERROR - CONFIG FILE NOT FOUND");
    let config_data: ConfigData = from_reader(config_file).expect("ERROR - FAILED TO DESERIALIZE CONFIG FILE");
    

    let firebase: Firebase = Firebase::new(&config_data.DATABASE_URL).expect("ERROR - CAN'T CONNECT TO DB");
    return firebase;
}

pub async fn test_query(firebase: &Firebase) {
        let firebase_ref = firebase.at("categories").at("car");
        let res: Vec<String> = firebase_ref.get::<Vec<String>>().await.expect("ERROR - QUERY NOT EXECUTED");

        println!("{:?}",res);
}

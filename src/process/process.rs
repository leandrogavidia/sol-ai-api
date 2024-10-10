use chromadb::v1::ChromaClient;
use chromadb::v1::collection::{ChromaCollection, GetResult, CollectionEntries};
use regex::Regex;
use serde_json::{from_str, json, Map};
use std::ffi::OsStr;
use std::{ fs, env };
use serde_json::Value;
use std::collections::HashMap;


pub fn split_content(file: &String) -> Vec<String> {
    let separator1 = "\n## ";
    let separator2 = "\n### ";

    // Replace all separator2 with separator1
    let file = file.replace(separator2, separator1);

    // Split the text using separator1
    let parts: Vec<String> = file.split(separator1).map(|i| i.to_string()).collect();

    parts
}

fn get_title(file: &str) -> Option<String> {

    let re = Regex::new(r"title:\s+(.+)\s+").unwrap();
    let match_ = re.captures(file);

    if let Some(captures) = match_ {
        let title = captures.get(1).map(|m| m.as_str().to_owned());
        return title;
    }

    None
}


pub fn process_files(dir_path: &str) {
    let mut document_id: u8 = 0;

    let current_dir = env::current_dir().unwrap();
    println!("{}{}", current_dir.to_str().unwrap(), dir_path);

    let client: ChromaClient = ChromaClient::new(Default::default());
    let collection: ChromaCollection = client.get_or_create_collection("solana_markdowns", None).unwrap();
    
    let files = fs::read_dir(format!("{}{}", current_dir.to_str().unwrap(), dir_path)).expect("Error reading dir path");
    println!("DIR FILES: {:?}", files);

    for f in files {
        let mk = f.unwrap();
        let path = mk.path();

        println!("\n\npath: {:?}\n\n",path);

        let file_name = path.file_name().unwrap();
        let file_extension = path.extension().unwrap();

        println!("is file: {}", path.is_file());
        println!("file_name: {:?}", path.file_name());
        println!("extension: {:?}", path.extension());

        if path.is_file() && file_extension == "md" {
            println!("It's a MK");

            let content = fs::read_to_string(path.clone()).expect("Error reading file");
            let shunks = split_content(&content);
            println!("shunks: {:?}", shunks);

            let mut ids: Vec<&str> = vec![];
            
            // generate_embeddings(shunks, file_name, &collection, &document_id);
        }
    }
} 

// pub fn generate_embeddings(chunks: Vec<String>, file_name: &OsStr, collection: &ChromaCollection, mut document_id: &u8) {
//     let mut metadatas: Vec<Map<String, Value>> = vec![];
//     let mut documents: Vec<&str> = vec![];

//     chunks.iter().map(|c| {
//         ids.push(document_id.to_string().as_str());
        
//         let metadata_objet = Map::new();
//         let value = r#"{"name": "John Doe", "age": 30, "vip": true}"#;
//         let metadata_value: Value = serde_json::from_str(value).unwrap();
//         metadata_objet.insert("file_name".to_string(), metadata_value);
//         documents.push(c);
//     });

//     let mut collection_entries= CollectionEntries {
//         ids,
//         metadatas: Some(metadatas),
//         documents: Some(documents),
//         embeddings: None
//     };

//     collection.add(collection_entries, None);
// }
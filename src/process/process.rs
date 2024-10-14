use chromadb::v1::collection::{
    ChromaCollection, CollectionEntries, GetOptions, GetResult, QueryOptions, QueryResult,
};
use chromadb::v1::embeddings::{self, EmbeddingFunction};
use chromadb::v1::ChromaClient;
use serde_json::Map;
use serde_json::Value;
use std::ffi::OsStr;
use std::{env, fs};

pub fn split_content(file: &String) -> Vec<String> {
    let separator1 = "\n## ";
    let separator2 = "\n### ";

    let file = file.replace(separator2, separator1);

    let parts: Vec<String> = file.split(separator1).map(|i| i.to_string()).collect();

    parts
}

pub fn process_files(dir_path: &str) {
    let document_id: u8 = 1;

    let current_dir = env::current_dir().unwrap();

    let client: ChromaClient = ChromaClient::new(Default::default());
    let collection: ChromaCollection = client
        .get_or_create_collection("solana_markdowns", None)
        .unwrap();

    let files = fs::read_dir(format!("{}{}", current_dir.to_str().unwrap(), dir_path))
        .expect("Error reading dir path");

    for f in files {
        let mk = f.unwrap();
        let path = mk.path();

        let file_name = path.file_name().unwrap();
        let file_extension = path.extension().unwrap();

        if path.is_file() && file_extension == "md" {
            let content = fs::read_to_string(path.clone()).expect("Error reading file");
            let shunks = split_content(&content);

            generate_embeddings(shunks, file_name, document_id, &collection);
        }
    }
}

pub fn generate_embeddings(
    chunks: Vec<String>,
    file_name: &OsStr,
    mut document_id: u8,
    collection: &ChromaCollection,
) {
    let string_ids = chunks
        .iter()
        .map(|_| {
            let id_string = format!("{}-{:?}", document_id, file_name);
            document_id += 1;
            id_string
        })
        .collect::<Vec<String>>();

    let ids: Vec<&str> = string_ids
        .iter()
        .map(|boxed_string| boxed_string.as_str())
        .collect();

    let metadatas: Vec<Map<String, Value>> = chunks
        .iter()
        .map(|_| {
            let mut metadata_objet = Map::new();
            metadata_objet.insert(
                "file_name".to_string(),
                serde_json::json!({ "file_name": file_name }),
            );
            metadata_objet
        })
        .collect();

    let documents: Vec<&str> = chunks.iter().map(|c| c.as_str()).collect();

        println!("{}", ids.len());
        println!("{}", metadatas.len());
        println!("{}", documents.len());

    let mut embeddings: Vec<Vec<f32>> = vec![];

    for _ in chunks.clone() {
        embeddings.push(vec![0.0_f32; 768])
    }

    let collection_entries = CollectionEntries {
        ids,
        metadatas: Some(metadatas),
        documents: Some(documents),
        embeddings: Some(embeddings),
    };

    let _ = collection.upsert(collection_entries, None).unwrap();
}

pub fn query_collection(query: &str) -> GetResult {
    let client: ChromaClient = ChromaClient::new(Default::default());
    let collection: ChromaCollection = client
        .get_or_create_collection("solana_markdowns", None)
        .unwrap();

    let where_document = serde_json::json!({
    "$contains": query
     });

    let get_query = GetOptions {
        ids: vec![],
        where_metadata: None,
        limit: Some(5),
        offset: None,
        where_document: Some(where_document),
        include: Some(vec!["documents".into(), "embeddings".into()]),
    };

    let result = collection.get(get_query).unwrap();
    println!("RESULT, {:?}", result);
    return result;
}

pub fn delete_collection() {
    let client: ChromaClient = ChromaClient::new(Default::default());

    let _ = client.delete_collection("solana_markdowns").unwrap();
}

pub fn query_example() {
    let client: ChromaClient = ChromaClient::new(Default::default());
    let collection: ChromaCollection = client
        .get_or_create_collection("solana_markdowns", None)
        .unwrap();
    // Upsert some embeddings with documents and no metadata.
    let collection_entries = CollectionEntries {
        ids: vec!["demo-id-1".into(), "demo-id-2".into()],
        embeddings: Some(vec![vec![0.0_f32; 768], vec![0.0_f32; 768]]),
        metadatas: None,
        documents: Some(vec![
            "Some document about 9 octopus recipies".into(),
            "Some other document about DCEU Superman Vs CW Superman".into(),
        ]),
    };

    let _result = collection.upsert(collection_entries, None).unwrap();

    let where_document = serde_json::json!({
    "$contains": "Superman"
     });

    let get_query = GetOptions {
        ids: vec![],
        where_metadata: None,
        limit: Some(1),
        offset: None,
        where_document: Some(where_document),
        include: Some(vec!["documents".into(), "embeddings".into()]),
    };
    let get_result: GetResult = collection.get(get_query).unwrap();
    println!("Get result: {:?}", get_result);
}

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::File;
use std::io::BufReader;
use std::iter;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
struct Identifier {
    id: String,
    uuid: String,
}

#[derive(Debug, Deserialize)]
struct Location {
    #[serde(rename = "$mid")]
    mid: i32,
    path: String,
    scheme: String,
}

#[derive(Debug, Deserialize)]
struct Metadata {
    id: String,
    publisherId: String,
    publisherDisplayName: String,
    targetPlatform: String,
    isApplicationScoped: Option<bool>,
    updated: bool,
    isPreReleaseVersion: bool,
    installedTimestamp: i64,
    preRelease: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct Extension {
    identifier: Identifier,
    version: String,
    location: Location,
    relativeLocation: String,
    metadata: Metadata,
}

fn main() {
    println!("\nSTARTING PIONEER.\n");

    let mut file_path: PathBuf = get_file_path();

    println!("Accessing Extensions Configuration: {:?}", &file_path);
    let response = get_extensions_list(file_path);
    match response {
        Ok(extensions) => list_extensions(extensions),
        Err(err) => eprintln!("Error: {}", err),
    }
}

fn get_file_path() -> PathBuf {
    let mut file_path: PathBuf = dirs::home_dir().unwrap();
    file_path.push(".vscode");
    file_path.push("extensions");
    file_path.push("extensions.json");
    file_path
}

fn get_extensions_list(file_path: PathBuf) -> Result<Vec<Extension>, Box<dyn std::error::Error>> {
    let file: File = File::open(&file_path)?;
    let reader: BufReader<File> = BufReader::new(file);
    let res: Result<Vec<Extension>, serde_json::Error> = serde_json::from_reader(reader);
    return res.map_err(|err| Box::new(err) as Box<dyn std::error::Error>);
}

fn list_extensions(extensions: Vec<Extension>) {
    println!("Currently Installed Extensions:\n");
    for extension in extensions.iter() {
        println!("ID: {}", &extension.identifier.id);
    }
    println!(" ");
}

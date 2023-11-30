mod get_extensions;
mod types;

use crate::get_extensions::extensions::*;
use std::path::PathBuf;

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

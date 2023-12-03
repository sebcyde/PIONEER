mod get_dirs;
mod get_extensions;
mod types;
mod update_extensions;

use crate::get_dirs::directories::*;
use crate::get_extensions::extensions::*;
use crate::update_extensions::update::update_all_extensions;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("\nSTARTING PIONEER.\n");

    create_pioneer_dir();
    create_pioneer_inner_dirs();
    let file_path: PathBuf = get_file_path();

    println!("Accessing Extensions Configuration: {:?}\n", &file_path);
    let response = get_extensions_list(file_path);
    match response {
        Ok(extensions) => {
            // list_extensions(&extensions).await;
            update_all_extensions(extensions).await;
        }
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

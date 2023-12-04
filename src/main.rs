mod get_dirs;
mod get_extensions;
mod types;
mod update_extensions;

use crate::get_dirs::directories::*;
use crate::get_extensions::extensions::*;
use crate::types::types::Extension;
use crate::update_extensions::update::update_all_extensions;
use std::path::PathBuf;

#[tokio::main]
async fn main() {
    println!("\nSTARTING PIONEER.\n");

    create_pioneer_dir();
    create_pioneer_inner_dirs();
    let file_path: PathBuf = get_file_path();

    println!("Accessing Extensions Configuration: {:?}\n", &file_path);
    let response = get_extensions_list(file_path.clone());

    match response {
        Ok(mut extensions) => {
            let new_vec: Vec<Extension> = extensions.split_off(10);
            update_all_extensions(new_vec).await;
        }
        Err(err) => eprintln!("Error: {}", err),
    }

    let response2 = get_extensions_list(file_path);
    match response2 {
        Ok(extensions) => {
            list_extensions(&extensions).await;
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

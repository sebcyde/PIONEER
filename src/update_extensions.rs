pub mod update {
    use std::{
        fs::{copy, File},
        io::{self, ErrorKind},
        path::{Path, PathBuf},
        process::{Command, Output},
    };

    use crate::{
        get_dirs::directories::{
            get_extension_backup_dir, get_extension_current_dir, get_extension_dir,
        },
        get_extensions::extensions::{download_latest_version, get_latest_version_link},
        types::types::Extension,
    };

    async fn move_to_backup_dir(
        extension_name: &str,
        extension_author: &str,
        version: &str,
    ) -> Result<(), std::io::Error> {
        println!("Creating Backup");
        let mut current_location: PathBuf =
            get_extension_current_dir(format!("{}-{}", &extension_author, &version).as_str());
        let mut backup_location: PathBuf = get_extension_backup_dir(&extension_name);

        current_location.push(".vsixmanifest");
        backup_location.push(".vsixmanifest");

        if let Err(err) = std::fs::copy(&current_location, &backup_location) {
            eprintln!("Error backing up {}: {}", &extension_name, err);
        } else {
            println!("Backup created successfully");
            return Ok(());
        }

        return Err(std::io::Error::new(
            ErrorKind::Other,
            "Failed to create backup. Aborting update.\n",
        ));
    }

    // pub async fn replace_current_config(
    //     extension_name: &str,
    //     extension_author: &str,
    //     version: &str,
    // ) -> Result<(), std::io::Error> {
    //     println!("Creating Backup");
    //     let mut new_file_location: PathBuf =
    //         get_extension_current_dir(format!("{}-{}", &extension_author, &version).as_str());

    //     let mut current_file_location: PathBuf = get_extension_dir(&extension_name);

    //     new_file_location.push(".vsixmanifest");
    //     new_file_location.push(".vsixmanifest");

    //     if let Err(err) = std::fs::copy(&current_location, &backup_location) {
    //         eprintln!("Error backing up {}: {}", &extension_name, err);
    //     } else {
    //         println!("Backup created successfully");
    //         return Ok(());
    //     }

    //     return Err(std::io::Error::new(
    //         ErrorKind::Other,
    //         "Failed to create backup. Aborting update.\n",
    //     ));
    // }

    // nightmARE - 0.0.3
    // cyberpunk - 1.2.10

    pub async fn update_single_extension(
        extension_name: &str,
        full_extensions_name: &str,
        version: &str,
    ) {
        println!("Updating {}", &extension_name);
        let extensions_dir: PathBuf = get_extension_dir(&extension_name);
        println!("Extension location: {:?}", &extensions_dir);

        _ = move_to_backup_dir(extension_name, full_extensions_name, version).await;
        // _ = replace_current_config(extension_name, full_extensions_name, version).await

        // println!("Update complete.\n");
    }

    async fn update_ext(extension_id: &str) {
        let code: &str = "C:/Users/SebCy/AppData/Local/Programs/Microsoft VS Code/Code.exe";
        println!("Code Path: {:?}\n", code);

        let output: Output = Command::new(code)
            .arg("--install-extension")
            .arg(extension_id)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            println!("{} updated successfully!", extension_id);
        } else {
            println!("Failed to update{}. Error: {:?}", extension_id, output);
        }
    }

    pub async fn update_all_extensions(extensions: Vec<Extension>) {
        for extension in extensions.iter() {
            let ext_details: Vec<&str> = extension.identifier.id.as_str().split('.').collect();
            println!("Name: {}", &ext_details[1]);
            println!("Author: {}", &ext_details[0]);
            println!("Current Version: {}", &extension.version);
            update_ext(extension.identifier.id.as_str()).await;

            // let ext_details: Vec<&str> = extension.identifier.id.as_str().split('.').collect();

            // println!("Name: {}", &ext_details[1]);
            // println!("Author: {}", &ext_details[0]);
            // println!("Current Version: {}", &extension.version);

            // let download_endpoint: String =
            //     get_latest_version_link(&ext_details[0], &ext_details[1]).await;
            // download_latest_version(download_endpoint, ext_details[1]).await;

            // update_single_extension(
            //     ext_details[1],
            //     &extension.identifier.id.as_str(),
            //     &extension.version,
            // )
            // .await;
        }
    }
}

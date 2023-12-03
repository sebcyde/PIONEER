pub mod update {
    use std::path::PathBuf;

    use crate::{
        get_dirs::directories::get_extension_dir,
        get_extensions::extensions::{download_latest_version, get_latest_version_link},
        types::types::Extension,
    };

    pub async fn update_single_extension(extension_name: &str) {
        println!("Updating {}", &extension_name);
        let extensions_dir: PathBuf = get_extension_dir(&extension_name);
        println!("Extension location: {:?}", &extensions_dir);

        println!("Update complete.\n");
    }

    pub async fn update_all_extensions(extensions: Vec<Extension>) {
        for extension in extensions.iter() {
            let ext_details: Vec<&str> = extension.identifier.id.as_str().split('.').collect();

            println!("Name: {}", &ext_details[1]);
            println!("Author: {}", &ext_details[0]);
            println!("Current Version: {}", &extension.version);

            let download_endpoint: String =
                get_latest_version_link(&ext_details[0], &ext_details[1]).await;
            download_latest_version(download_endpoint, ext_details[1]).await;

            update_single_extension(ext_details[1]).await;
        }
    }
}

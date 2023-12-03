pub mod extensions {

    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use reqwest::{Client, Error, Response};
    use std::io::copy;

    use crate::{
        get_dirs::directories::{create_extension_dir, get_extension_dir, get_pioneer_dir},
        types::types::Extension,
    };

    pub fn get_extensions_list(
        file_path: PathBuf,
    ) -> Result<Vec<Extension>, Box<dyn std::error::Error>> {
        let file: File = File::open(&file_path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let res: Result<Vec<Extension>, serde_json::Error> = serde_json::from_reader(reader);
        return res.map_err(|err| Box::new(err) as Box<dyn std::error::Error>);
    }

    pub async fn get_latest_version_link(author: &str, name: &str) -> String {
        let ext_dir: PathBuf = get_extension_dir(&name);
        create_extension_dir(&ext_dir);

        let api_endpoint: String =
            format!("https://ms-vscode.gallery.vsassets.io/_apis/public/gallery/publisher/{}/extension/{}/latest/assetbyname/Microsoft.VisualStudio.Services.VSIXPackage", &author, &name);

        return api_endpoint;
    }

    pub async fn download_latest_version(link: String, extension_name: &str) {
        let client: Client = Client::new();
        let res: Result<Response, Error> = client.get(link).send().await;
        let response: Response = res.unwrap();

        if response.status().is_success() {
            let download_dir: PathBuf = get_extension_dir(&extension_name);
            println!("Downloading update file...");

            let path: PathBuf =
                download_dir.join("Microsoft.VisualStudio.Services.VSIXPackage.VSIXPackage");

            let mut output_file: File = File::create(path).unwrap();
            let res_bytes = response.bytes().await.unwrap();
            let _ = copy(&mut res_bytes.as_ref(), &mut output_file);

            println!("Download successful.\n");
        } else {
            println!(
                "Failed to download the file. Status code: {}",
                response.status()
            );
        }
    }

    pub async fn list_extensions(extensions: &Vec<Extension>) {
        println!("Currently Installed Extensions:\n");

        for extension in extensions.iter() {
            let ext_details: Vec<&str> = extension.identifier.id.as_str().split('.').collect();
            println!("Author: {}", &ext_details[0]);
            println!("Name: {}", &ext_details[1]);
            println!("Current Version: {}", &extension.version);

            let download_endpoint: String =
                get_latest_version_link(&ext_details[0], &ext_details[1]).await;
            download_latest_version(download_endpoint, ext_details[1]).await;
        }
    }
}

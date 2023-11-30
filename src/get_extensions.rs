pub mod extensions {

    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    use crate::types::types::Extension;

    pub fn get_extensions_list(
        file_path: PathBuf,
    ) -> Result<Vec<Extension>, Box<dyn std::error::Error>> {
        let file: File = File::open(&file_path)?;
        let reader: BufReader<File> = BufReader::new(file);
        let res: Result<Vec<Extension>, serde_json::Error> = serde_json::from_reader(reader);
        return res.map_err(|err| Box::new(err) as Box<dyn std::error::Error>);
    }

    pub fn list_extensions(extensions: Vec<Extension>) {
        println!("Currently Installed Extensions:\n");
        for extension in extensions.iter() {
            println!("ID: {}", &extension.identifier.id);
        }
        println!(" ");
    }
}

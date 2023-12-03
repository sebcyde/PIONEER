pub mod directories {

    use std::path::Path;
    use std::path::PathBuf;

    pub fn create_pioneer_dir() {
        let mut home: PathBuf = dirs::home_dir().unwrap();
        home.push("Documents");
        home.push("PIONEER");
        _ = std::fs::create_dir_all(home);
    }

    pub fn create_pioneer_inner_dirs() {
        let mut log_dir: PathBuf = get_pioneer_dir();
        let mut bug_dir: PathBuf = get_pioneer_dir();
        let mut download_dir: PathBuf = get_pioneer_dir();

        download_dir.push("Files");
        log_dir.push("Logs");
        bug_dir.push("Bugs");

        _ = std::fs::create_dir_all(download_dir);
        _ = std::fs::create_dir_all(log_dir);
        _ = std::fs::create_dir_all(bug_dir);
    }

    pub fn get_pioneer_dir() -> PathBuf {
        let mut home: PathBuf = dirs::home_dir().unwrap();
        home.push("Documents");
        home.push("PIONEER");
        return home;
    }

    pub fn create_extension_dir(extensions_dir: &PathBuf) {
        if !Path::new(&extensions_dir).exists() {
            println!("Creating extension directory.");
            _ = std::fs::create_dir_all(extensions_dir);
            println!("Directory creation successful.");
        } else {
            println!("Extension directory already exists. Skipping creation.");
        }
    }

    pub fn get_extension_dir(extension_name: &str) -> PathBuf {
        let mut root: PathBuf = get_download_dir();
        root.push(extension_name);
        return root;
    }

    pub fn get_download_dir() -> PathBuf {
        let mut home: PathBuf = dirs::home_dir().unwrap();
        home.push("Documents");
        home.push("PIONEER");
        home.push("Files");

        return home;
    }
}

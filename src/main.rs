mod args;
mod fetch;
mod package;

use crate::args::get_matches;
use crate::fetch::fetch_description;
use crate::package::PackageJson;

use std::path::Path;
use std::fs::File;
use std::io::Read;

#[tokio::main]
async fn main() {
    let matches = get_matches();

    let file_path = matches.get_one::<String>("file").map_or("package.json", String::as_str);

    if !Path::new(file_path).exists() {
        eprintln!("File {} does not exist", file_path);
        std::process::exit(1);
    }

    let mut file = File::open(file_path).expect("package.json not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    let package_json: PackageJson = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    if let Some(dependencies) = package_json.dependencies {
        for (name, _) in dependencies {
            if let Some(description) = fetch_description(&name).await {
                println!("{}: {}", name, description);
            } else {
                println!("{}: No description found", name);
            }
        }
    }

    if let Some(dev_dependencies) = package_json.devDependencies {
        for (name, _) in dev_dependencies {
            if let Some(description) = fetch_description(&name).await {
                println!("{}: {}", name, description);
            } else {
                println!("{}: No description found", name);
            }
        }
    }
}

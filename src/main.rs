mod args;
mod fetch;
mod package;

use crate::args::get_matches;
use crate::fetch::fetch_description;
use crate::package::PackageJson;

use std::path::Path;
use std::fs::File;
use std::io::Read;
use colored::*;
use regex::Regex;

fn remove_html_tags(text: &str) -> String {
    let re = Regex::new(r"<[^>]*>").unwrap();
    re.replace_all(text, "").to_string()
}

#[tokio::main]
async fn main() {
    let matches = get_matches();

    let file_path = matches.get_one::<String>("file").map_or("package.json", String::as_str);

    if !Path::new(file_path).exists() {
        eprintln!("{}", "======================================= \n 😩 No package.json found!\n=======================================".red().bold());
        std::process::exit(1);
    }

    let mut file = File::open(file_path).expect("package.json not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("something went wrong reading the file");

    let package_json: PackageJson = serde_json::from_str(&contents).expect("JSON was not well-formatted");

    if let Some(dependencies) = package_json.dependencies {
        for (name, _) in dependencies {
            let key_colored = name.black().bold().on_magenta(); 
            if let Some(description) = fetch_description(&name).await {
                let cleaned_description = remove_html_tags(&description);
                println!("{}: {}", key_colored, cleaned_description);
            } else {
                println!("{}: No description found", key_colored);
            }
        }
    }

    if let Some(dev_dependencies) = package_json.devDependencies {
        for (name, _) in dev_dependencies {
            let key_colored = name.black().bold().on_blue(); 
            if let Some(description) = fetch_description(&name).await {
                let cleaned_description = remove_html_tags(&description);
                println!("{}: {}", key_colored, cleaned_description);
            } else {
                println!("{}: No description found", key_colored);
            }
        }
    }
}

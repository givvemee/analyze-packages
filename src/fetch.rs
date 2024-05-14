// get package description
use reqwest;
use serde_json;

pub async fn fetch_description(package_name: &str) -> Option<String> {
    let url = format!("https://registry.npmjs.org/{}", package_name);
    let response = reqwest::get(&url).await.ok()?;
    let json: serde_json::Value = response.json().await.ok()?;
    json["description"].as_str().map(|s| s.to_string())
}

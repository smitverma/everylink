use reqwest::blocking::Client;
use reqwest::StatusCode;
use colored::*;

pub fn check_if_page_exists(url : String) -> Result<(), Box<dyn std::error::Error>> {

    let client = Client::new();
    let response = client.get(url.clone()).send()?;

    match response.status() {
        StatusCode::OK => println!("{}", format!("{} -> 200 / OK", url.clone()).green()),
        StatusCode::NOT_FOUND => println!("{}", format!("{} -> 404 / NOT FOUND", url).clone().red()),
        StatusCode::UNAUTHORIZED => println!("{}", format!("{} -> 401 / UNAUTHORIZED", url.clone()).yellow()),
        _ => {println!("Error!")}
    }

    Ok(())
}

use std::fs::File;
use std::io;
use std::io::Write;
use std::process;
use std::fs;
use std::result;

use keyring::error::Error;

use bsky_sdk::BskyAgent;
use atrium_api::types::string::Datetime;
use atrium_api::app::bsky::feed::get_timeline::ParametersData;
use bsky_sdk::agent::config::{Config, FileStore};
use atrium_api::app::bsky::feed::post::RecordData as PostRecordData;
use serde_json::from_value;



// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, login])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}












#[tauri::command]
async fn login(uname: String, pwd: String) -> Result<(), String> {
//    try_login(uname, pwd)
  //      .await
    //    .map_err(|e| e.to_string())
    println!("login {uname} {pwd}");
    Ok(())
}


async fn try_login(uname: &str, pass: &str) -> Result<(), Box<dyn std::error::Error>> {
    let service = "cli_sky";
    let username = "user";
    let entry = keyring::Entry::new(service, username)?;

    let pwd = match entry.get_password() {
        Ok(secret) => secret,
        Err(Error::NoEntry) => {
            // Prompt user for password, or use provided `pass`
            // Here, we store the password in the keyring for next time
            entry.set_password(pass)?;
            pass.to_string()
        }
        Err(e) => {
            eprintln!("Failed to get password: {}", e);
            return Err(Box::new(e));
        }
    };

    println!("Got secret: {pwd}");

    let mut file = File::create("config.json")?;
    file.write_all(pwd.as_bytes())?;

    let agent = BskyAgent::builder()
    .config(Config::load(&FileStore::new("config.json")).await?)
    .build()
    .await?;

    match start_session(agent).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("\nLogin failed. Please enter new details.");
            println!("{e}");
            // Here, you should probably prompt for new credentials,
            // but for now we'll just return the error
            Err(e)
        }
    }

}

async fn start_session(agent: BskyAgent) -> Result<(), Box<dyn std::error::Error>> {
    save_session(&agent).await?;

    Ok(())

}

async fn save_session(agent: &BskyAgent) -> Result<(), Box<dyn std::error::Error>> {
    agent.to_config()
    .await
    .save(&FileStore::new("config.json"))
    .await?;
    println!("Session saved to config.json");

    //deserialize the json
    let config = fs::read_to_string("config.json")?;

    //create an entry
    let service = "cli_sky";
    let username = "user";
    let entry = keyring::Entry::new(service, username)?;
    entry.set_password(&config)?;

    //delete the config.json file
    fs::remove_file("config.json")?;

    Ok(())
}

async fn user_login() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}







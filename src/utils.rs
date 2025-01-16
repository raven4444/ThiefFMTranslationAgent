use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::secure_storage::SecureStorage;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
use std::process::exit;

#[derive(Deserialize)]
struct GithubRelease {
    tag_name: String,
    html_url: String,
}

pub fn read_api_key_from_console() -> io::Result<String> {
    print!("{}{}{}", COLOR_GREEN, ENTER_API_KEY, COLOR_RESET);
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    Ok(api_key.trim().to_string())
}

pub fn wait_for_key_press() -> io::Result<()> {
    print!("{}{}{}", COLOR_GREEN, CONTINUE, COLOR_RESET);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

pub fn is_valid_api_key(key: &str) -> bool {
    key.starts_with(API_KEY_PREFIX)
}

pub fn mask_api_key(api_key: &str) -> String {
    if api_key.len() > 4 {
        format!("{}{}", &api_key[..6], CENSORSHIP)
    } else {
        CENSORSHIP.to_string()
    }
}

pub async fn handle_invalid_key(storage: &SecureStorage, message: &str) -> io::Result<String> {
    println!("{}{}{}", COLOR_RED, message, COLOR_RESET);
    let new_key = get_valid_api_key().await?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

pub async fn get_valid_api_key() -> io::Result<String> {
    loop {
        let key = read_api_key_from_console()?;
        let openai_client = OpenAIClient::new(key.clone());
        if is_valid_api_key(&key) && openai_client.verify_key().await {
            drop(openai_client);
            println!("{}{}{}", COLOR_GREEN, VALID_KEY, COLOR_RESET);
            return Ok(key);
        }
        println!("{}{}{}", COLOR_RED, INVALID_KEY_WARNING, COLOR_RESET);
    }
}

pub fn store_api_key(storage: &SecureStorage, key: &str) -> io::Result<()> {
    storage.store_api_key(key)?;
    println!("{}{}{}", COLOR_GREEN, KEY_SAVED, COLOR_RESET);
    Ok(())
}

pub async fn retrieve_existing_key(storage: &SecureStorage) -> io::Result<String> {
    match storage.get_api_key() {
        Ok(key) => {
            if is_valid_api_key(&key) {
                println!("{}{}{}", COLOR_GREEN, KEY_RETRIEVED, COLOR_RESET);
                Ok(key)
            } else {
                handle_invalid_key(storage, KEY_CORRUPTED).await
            }
        }
        Err(e) => handle_invalid_key(storage, &format!("{} {}", KEY_RETRIEVAL_ERROR, e)).await,
    }
}

pub async fn handle_new_key(storage: &SecureStorage) -> io::Result<String> {
    let new_key = get_valid_api_key().await?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

pub fn ask_default_true() -> io::Result<bool> {
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();
    Ok(input.is_empty() || input.to_lowercase() == YES)
}

pub async fn check_version() -> Result<(), Box<dyn Error>> {
    let current_version = env!("CARGO_PKG_VERSION");
    let app_name = env!("CARGO_PKG_NAME");
    println!(t
        "{}{} {}{}",
        COLOR_GREEN, app_name, current_version, COLOR_RESET
    );
    let client = reqwest::Client::builder()
        .user_agent(app_name.to_owned() + current_version)
        .build()?;

    let response = client
        .get(RELEASES_URL)
        .send()
        .await?
        .json::<GithubRelease>()
        .await?;

    let latest_version = response.tag_name.trim_start_matches(VERSION_PREFIX);

    if latest_version != current_version {
        println!("{}{}{}", COLOR_YELLOW, NEW_VERSION, latest_version);
        println!("{}{}", UPDATE_APP, COLOR_RESET);
        let open_release = ask_default_true()?;
        if open_release {
            webbrowser::open(&response.html_url)?;
            wait_for_key_press()?;
            exit(0);
        }
    }

    Ok(())
}

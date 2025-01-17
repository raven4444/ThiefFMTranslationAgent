use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::secure_storage::SecureStorage;
use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use std::process::exit;
use chardet::detect;
use encoding_rs::Encoding;

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
    println!(
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

pub fn get_fm_directory_path() -> io::Result<String> {
    loop {
        print!("{}{}{} ", COLOR_GREEN, PATH_TO_FM, COLOR_RESET);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input_path = input.trim();
        let path = Path::new(input_path);
        if !path.exists() {
            println!("{}{}{}", COLOR_RED, MISSING_DIRECTORY, COLOR_RESET);
            continue;
        }

        if !path.is_dir() {
            println!("{}{}{}", COLOR_RED, INVALID_PATH, COLOR_RESET);
            continue;
        }
        let has_mis_file = fs::read_dir(path)?.filter_map(Result::ok).any(|entry| {
            entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == MIS_EXTENSION)
                .unwrap_or(false)
        });

        if !has_mis_file {
            println!("{}{}{}", COLOR_RED, MISSING_FM_FILES, COLOR_RESET);
            continue;
        }
        let directory = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&*String::new())
            .to_string();
        println!(
            "{}{}{}{}",
            COLOR_GREEN, MISSION_DIRECTORY, directory, COLOR_RESET
        );
        break Ok(input_path.to_string());
    }
}

pub fn read_file_content(file_path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    // Read the file into bytes first
    let mut raw_bytes = Vec::new();
    let mut file = fs::File::open(file_path)?;
    file.read_to_end(&mut raw_bytes)?;

    // Detect the encoding
    let detect_result = detect(&raw_bytes);
    let encoding_name = detect_result.0;

    // Get the encoding from the detected name
    let encoding = Encoding::for_label(encoding_name.as_bytes())
        .unwrap_or(encoding_rs::WINDOWS_1252);

    // Convert to UTF-8
    let (cow, _, had_errors) = encoding.decode(&raw_bytes);

    if had_errors {
        println!("Warning: Some characters couldn't be decoded properly using detected encoding: {}", encoding_name);
    }

    Ok(cow.into_owned())
}

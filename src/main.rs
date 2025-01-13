mod constants;
mod openai_client;
mod secure_storage;

use crate::openai_client::OpenAIClient;
use crate::secure_storage::SecureStorage;
use constants::*;
use std::io::{self, Write};

fn read_api_key_from_console() -> io::Result<String> {
    print!("{}{}{}", COLOR_GREEN, ENTER_API_KEY, COLOR_RESET);
    io::stdout().flush()?;
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    Ok(api_key.trim().to_string())
}

fn wait_for_key_press() -> io::Result<()> {
    print!("{}{}{}", COLOR_GREEN, CONTINUE, COLOR_RESET);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}

fn is_valid_api_key(key: &str) -> bool {
    key.starts_with(API_KEY_PREFIX)
}

fn mask_api_key(api_key: &str) -> String {
    if api_key.len() > 4 {
        format!("{}{}", &api_key[..4], CENSORSHIP)
    } else {
        CENSORSHIP.to_string()
    }
}

async fn handle_invalid_key(storage: &SecureStorage, message: &str) -> io::Result<String> {
    println!("{}{}{}", COLOR_RED, message, COLOR_RESET);
    let new_key = get_valid_api_key().await?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

async fn get_valid_api_key() -> io::Result<String> {
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

fn store_api_key(storage: &SecureStorage, key: &str) -> io::Result<()> {
    storage.store_api_key(key)?;
    println!("{}{}{}", COLOR_GREEN, KEY_SAVED, COLOR_RESET);
    Ok(())
}

async fn retrieve_existing_key(storage: &SecureStorage) -> io::Result<String> {
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

async fn handle_new_key(storage: &SecureStorage) -> io::Result<String> {
    println!("{}{}{}", COLOR_YELLOW, KEY_NOT_FOUND, COLOR_RESET);
    let new_key = get_valid_api_key().await?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let storage = SecureStorage::new()?;

    let api_key = if storage.api_key_exists() {
        println!("{}{}{}", COLOR_GREEN, KEY_FOUND, COLOR_RESET);
        retrieve_existing_key(&storage).await?
    } else {
        handle_new_key(&storage).await?
    };

    let masked_key = mask_api_key(&api_key);
    println!("{}{} {}{}", COLOR_YELLOW, USE_KEY, masked_key, COLOR_RESET);
    let openai_client = OpenAIClient::new(api_key);
    //todo
    wait_for_key_press()?;
    Ok(())
}

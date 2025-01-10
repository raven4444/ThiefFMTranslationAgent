mod constants;
mod secure_storage;

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

fn handle_invalid_key(storage: &SecureStorage, message: &str) -> io::Result<String> {
    println!("{}{}{}", COLOR_RED, message, COLOR_RESET);
    let new_key = get_valid_api_key()?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

fn get_valid_api_key() -> io::Result<String> {
    loop {
        let key = read_api_key_from_console()?;
        if is_valid_api_key(&key) {
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

fn retrieve_existing_key(storage: &SecureStorage) -> io::Result<String> {
    match storage.get_api_key() {
        Ok(key) => {
            if is_valid_api_key(&key) {
                println!("{}{}{}", COLOR_GREEN, KEY_RETRIEVED, COLOR_RESET);
                Ok(key)
            } else {
                handle_invalid_key(storage, KEY_CORRUPTED)
            }
        }
        Err(e) => handle_invalid_key(storage, &format!("{} {}", KEY_RETRIEVAL_ERROR, e)),
    }
}

fn handle_new_key(storage: &SecureStorage) -> io::Result<String> {
    println!("{}{}{}", COLOR_YELLOW, KEY_NOT_FOUND, COLOR_RESET);
    let new_key = get_valid_api_key()?;
    store_api_key(storage, &new_key)?;
    Ok(new_key)
}

fn main() -> io::Result<()> {
    let storage = SecureStorage::new()?;

    let api_key = if storage.api_key_exists() {
        println!("{}{}{}", COLOR_GREEN, KEY_FOUND, COLOR_RESET);
        retrieve_existing_key(&storage)?
    } else {
        handle_new_key(&storage)?
    };

    let masked_key = mask_api_key(&api_key);
    println!("{}{} {}{}", COLOR_YELLOW, USE_KEY, masked_key, COLOR_RESET);
    //todo
    wait_for_key_press()?;
    Ok(())
}

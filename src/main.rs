mod constants;
mod openai_client;
mod secure_storage;
mod prompt_service;
mod utils;

use std::error::Error;
use crate::openai_client::OpenAIClient;
use crate::secure_storage::SecureStorage;
use constants::*;
use crate::prompt_service::PromptService;
use crate::utils::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let storage = SecureStorage::new()?;

    let api_key = if storage.api_key_exists() {
        println!("{}{}{}", COLOR_GREEN, KEY_FOUND, COLOR_RESET);
        retrieve_existing_key(&storage).await?
    } else {
        println!("{}{}{}", COLOR_YELLOW, KEY_NOT_FOUND, COLOR_RESET);
        handle_new_key(&storage).await?
    };

    let masked_key = mask_api_key(&api_key);
    println!("{}{} {}{}", COLOR_YELLOW, USE_KEY, masked_key, COLOR_RESET);
    let use_current_key = ask_use_current_key()?;
    if !use_current_key {
        let api_key = handle_new_key(&storage).await?;
        let masked_key = mask_api_key(&api_key);
        println!("{}{} {}{}", COLOR_YELLOW, USE_KEY, masked_key, COLOR_RESET);
    }
    let openai_client = OpenAIClient::new(api_key);
    let prompt_service = PromptService::initialize().await?;
    prompt_service.print_prompts_overview();
    //todo
    wait_for_key_press()?;
    Ok(())
}
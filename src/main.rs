mod constants;
mod openai_client;
mod prompt_service;
mod secure_storage;
mod translation_cache_service;
mod translation_helper;
mod utils;

use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::secure_storage::SecureStorage;
use crate::translation_helper::TranslationHelper;
use crate::utils::{
    ask_default_true, check_version, handle_new_key, mask_api_key, retrieve_existing_key,
    wait_for_key_press,
};
use constants::*;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    check_version().await?;

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
    print!("{}{}{}", COLOR_GREEN, REUSE_KEY, COLOR_RESET);
    let use_current_key = ask_default_true()?;
    if !use_current_key {
        let api_key = handle_new_key(&storage).await?;
        let masked_key = mask_api_key(&api_key);
        println!("{}{} {}{}", COLOR_YELLOW, USE_KEY, masked_key, COLOR_RESET);
    }
    let openai_client = OpenAIClient::new(api_key);
    let prompt_service = PromptService::initialize().await?;
    prompt_service.print_prompts_overview();
    TranslationHelper::new(openai_client, prompt_service)
        .run()
        .await?;
    println!("{}{}{}", COLOR_GREEN, LAST_INFO, COLOR_RESET);
    wait_for_key_press()?;
    Ok(())
}

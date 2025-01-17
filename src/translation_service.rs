use crate::cache_service::CacheService;
use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::{get_fm_directory_path, wait_for_key_press};

pub struct TranslationService {
    openai_client: OpenAIClient,
    prompt_service: PromptService,
    path: String,
    cache: Option<CacheService>,
}

impl TranslationService {
    pub fn new(openai_client: OpenAIClient, prompt_service: PromptService) -> Self {
        TranslationService {
            openai_client,
            prompt_service,
            path: String::new(),
            cache: None,
        }
    }

    pub fn run(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.path = get_fm_directory_path()?;
        self.cache = Option::from(CacheService::new(&self.path)?);
        if self.cache.is_none() {
            println!("{}", ISSUE_WITH_CACHE);
            wait_for_key_press()?;
            std::process::exit(1);
        }
        //todo
        Ok("Processing completed".to_string())
    }
}

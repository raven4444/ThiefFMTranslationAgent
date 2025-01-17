use crate::cache_service::CacheService;
use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::{get_fm_directory_path, read_file_content, wait_for_key_press};
use std::fs;
use std::path::Path;

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
        self.populate_cache_objects()?;
        //todo
        Ok(String::new())
    }

    fn populate_cache_objects(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = Path::new(&self.path);

        let strings_dir = fs::read_dir(base_path)?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == "strings")
            .ok_or("Strings directory not found")?;

        let english_dir = fs::read_dir(strings_dir.path())?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == "english")
            .ok_or("English directory not found")?;

        let obj_names_file = fs::read_dir(english_dir.path())?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == "objnames.str")
            .ok_or("objnames.str file not found")?;

        let original_path = obj_names_file.path().to_string_lossy().to_string();

        if let Some(cache) = &mut self.cache {
            if let Some(_) = cache.find_by_original_path(&original_path)? {
                return Ok(());
            }

            let filename = obj_names_file.file_name().to_string_lossy().to_string();
            let english_str = english_dir.file_name().to_string_lossy().to_string();
            let translated_path = original_path.replace(&english_str, "polish");
            let original_content = read_file_content(&obj_names_file.path())?;

            cache.insert(
                &filename,
                &original_path,
                &translated_path,
                &original_content,
                None,
            )?;
        }

        Ok(())
    }
}

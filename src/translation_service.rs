use crate::cache_service::CacheService;
use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::{get_fm_directory_path, read_file_content, wait_for_key_press};
use std::fs;
use std::path::{Path, PathBuf};

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
        self.populate_cache()?;
        //todo
        Ok(String::new())
    }

    fn populate_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = PathBuf::from(&self.path);

        self.process_objnames_file(&base_path)?;
        self.process_goals_files(&base_path)?;

        Ok(())
    }

    fn process_objnames_file(
        &mut self,
        base_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
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

        self.process_file(&obj_names_file.path(), "objnames.str")?;

        Ok(())
    }

    fn process_goals_files(&mut self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let interface_dir = fs::read_dir(base_path)?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == "intrface")
            .ok_or("Interface directory not found")?;

        for entry in fs::read_dir(interface_dir.path())? {
            let entry = entry?;
            let dir_name = entry.file_name().to_string_lossy().to_lowercase();

            if dir_name.starts_with("miss") && entry.path().is_dir() {
                let english_dir = fs::read_dir(entry.path())?
                    .filter_map(Result::ok)
                    .find(|e| e.file_name().to_string_lossy().to_lowercase() == "english");

                if let Some(english_dir) = english_dir {
                    let goals_file = fs::read_dir(english_dir.path())?
                        .filter_map(Result::ok)
                        .find(|e| e.file_name().to_string_lossy().to_lowercase() == "goals.str");

                    if let Some(goals_file) = goals_file {
                        let mission_dir = entry.file_name().to_string_lossy().to_string(); // Convert to owned String
                        let custom_filename = format!("{}/english/goals.str", mission_dir);

                        self.process_file(&goals_file.path(), &custom_filename)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn process_file(
        &mut self,
        file_path: &Path,
        custom_filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let original_path = file_path.to_string_lossy().to_string();

        if let Some(cache) = &mut self.cache {
            if let Some(_) = cache.find_by_original_path(&original_path)? {
                return Ok(());
            }

            let translated_path = original_path.replace("english", "polish");

            let original_content = read_file_content(file_path)?;

            cache.insert(
                custom_filename,
                &original_path,
                &translated_path,
                &original_content,
                None,
            )?;
        }

        Ok(())
    }
}

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
        self.process_books_files(&base_path)?;
        Ok(())
    }

    fn process_objnames_file(
        &mut self,
        base_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let strings_dir = fs::read_dir(base_path)?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == STRINGS_DIR)
            .ok_or(DIRECTORY_NOT_FOUND.to_owned() + STRINGS_DIR)?;

        let english_dir = fs::read_dir(strings_dir.path())?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == ENGLISH_DIR)
            .ok_or(DIRECTORY_NOT_FOUND.to_owned() + ENGLISH_DIR)?;

        let obj_names_file = fs::read_dir(english_dir.path())?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == OBJNAMES_FILE)
            .ok_or(FILE_NOT_FOUND.to_owned() + OBJNAMES_FILE)?;

        self.process_file(&obj_names_file.path(), OBJNAMES_FILE)?;

        Ok(())
    }

    fn process_goals_files(&mut self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let interface_dir = fs::read_dir(base_path)?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == INTRFACE_DIR)
            .ok_or(DIRECTORY_NOT_FOUND.to_owned() + INTRFACE_DIR)?;

        for entry in fs::read_dir(interface_dir.path())? {
            let entry = entry?;
            let dir_name = entry.file_name().to_string_lossy().to_lowercase();

            if dir_name.starts_with(MISS_DIR) && entry.path().is_dir() {
                let english_dir = fs::read_dir(entry.path())?
                    .filter_map(Result::ok)
                    .find(|e| e.file_name().to_string_lossy().to_lowercase() == ENGLISH_DIR);

                if let Some(english_dir) = english_dir {
                    let goals_file = fs::read_dir(english_dir.path())?
                        .filter_map(Result::ok)
                        .find(|e| e.file_name().to_string_lossy().to_lowercase() == GOALS_FILE);

                    if let Some(goals_file) = goals_file {
                        let mission_dir = entry.file_name().to_string_lossy().to_string();
                        let custom_filename = format!("{}{}", mission_dir, PATH_TO_GOALS);

                        self.process_file(&goals_file.path(), &custom_filename)?;
                    }
                }
            }
        }

        Ok(())
    }

    fn process_books_files(&mut self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let books_dir = fs::read_dir(base_path)?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == BOOKS_DIR)
            .ok_or(DIRECTORY_NOT_FOUND.to_owned() + BOOKS_DIR)?;

        let english_dir = fs::read_dir(books_dir.path())?
            .filter_map(Result::ok)
            .find(|entry| entry.file_name().to_string_lossy().to_lowercase() == ENGLISH_DIR);

        let scan_dir = if let Some(eng_dir) = english_dir {
            eng_dir.path()
        } else {
            books_dir.path()
        };

        for entry in fs::read_dir(scan_dir)? {
            let entry = entry?;
            if entry
                .path()
                .extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext.to_lowercase() == STR_EXTENSION)
                .unwrap_or(false)
            {
                let filename = entry.file_name().to_string_lossy().to_string();
                self.process_file(&entry.path(), &filename)?;
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

            let translated_path = original_path.replace(ENGLISH_DIR, POLISH_DIR);

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

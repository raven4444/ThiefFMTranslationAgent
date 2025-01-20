use crate::cache_service::CacheService;
use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::{get_fm_directory_path, read_file_content, wait_for_key_press};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug)]
enum FileSystemItem {
    Directory(String),
    File(String),
}

impl FileSystemItem {
    fn name(&self) -> &str {
        match self {
            FileSystemItem::Directory(name) | FileSystemItem::File(name) => name,
        }
    }

    fn not_found_message(&self) -> String {
        match self {
            FileSystemItem::Directory(_) => DIRECTORY_NOT_FOUND.to_string(),
            FileSystemItem::File(_) => FILE_NOT_FOUND.to_string(),
        }
    }
}

pub struct TranslationService {
    openai_client: OpenAIClient,
    prompt_service: PromptService,
    path: String,
    cache: Option<CacheService>,
}

trait FileProcessor {
    fn process(
        &self,
        base_path: &Path,
        service: &mut TranslationService,
    ) -> Result<(), Box<dyn std::error::Error>>;
}

struct ObjNamesProcessor;
struct GoalsProcessor;
struct BooksProcessor;

impl TranslationService {
    pub fn new(openai_client: OpenAIClient, prompt_service: PromptService) -> Self {
        TranslationService {
            openai_client,
            prompt_service,
            path: String::new(),
            cache: None,
        }
    }

    pub async fn run(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.initialize()?;
        self.populate_cache()?;
        self.cache
            .as_mut()
            .unwrap()
            .translate_cache(&self.openai_client, &self.prompt_service)
            .await?;

        Ok(String::new())
    }

    fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.path = get_fm_directory_path()?;
        self.cache = Option::from(CacheService::new(&self.path)?);

        if self.cache.is_none() {
            println!("{}", ISSUE_WITH_CACHE);
            wait_for_key_press()?;
            std::process::exit(1);
        }
        Ok(())
    }

    fn populate_cache(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let base_path = PathBuf::from(&self.path);
        let processors: Vec<Box<dyn FileProcessor>> = vec![
            Box::new(ObjNamesProcessor),
            Box::new(GoalsProcessor),
            Box::new(BooksProcessor),
        ];

        for processor in processors {
            processor.process(&base_path, self)?;
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
            if cache.find_by_original_path(&original_path)?.is_some() {
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

    fn find_item(
        &self,
        path: &Path,
        item: FileSystemItem,
    ) -> Result<PathBuf, Box<dyn std::error::Error>> {
        fs::read_dir(path)?
            .filter_map(Result::ok)
            .find(|entry| {
                let name_matches = entry.file_name().to_string_lossy().to_lowercase()
                    == item.name().to_lowercase();
                match item {
                    FileSystemItem::Directory(_) => name_matches && entry.path().is_dir(),
                    FileSystemItem::File(_) => name_matches && entry.path().is_file(),
                }
            })
            .map(|entry| entry.path())
            .ok_or_else(|| format!("{}{}", item.not_found_message(), item.name()).into())
    }

    fn find_dir(&self, path: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        self.find_item(path, FileSystemItem::Directory(name.to_string()))
    }

    fn find_file(&self, path: &Path, name: &str) -> Result<PathBuf, Box<dyn std::error::Error>> {
        self.find_item(path, FileSystemItem::File(name.to_string()))
    }
}

impl FileProcessor for ObjNamesProcessor {
    fn process(
        &self,
        base_path: &Path,
        service: &mut TranslationService,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let strings_dir = service.find_dir(base_path, STRINGS_DIR)?;
        let english_dir = service
            .find_dir(&strings_dir, ENGLISH_DIR)
            .unwrap_or_else(|_| strings_dir);
        if let Ok(obj_names_path) = service.find_file(&english_dir, OBJNAMES_FILE) {
            service.process_file(&obj_names_path, OBJNAMES_FILE)?;
        }
        Ok(())
    }
}

impl FileProcessor for GoalsProcessor {
    fn process(
        &self,
        base_path: &Path,
        service: &mut TranslationService,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let interface_dir = service.find_dir(base_path, INTRFACE_DIR)?;

        for entry in fs::read_dir(interface_dir)? {
            let entry = entry?;
            let dir_name = entry.file_name().to_string_lossy().to_lowercase();

            if dir_name.starts_with(MISS_DIR) && entry.path().is_dir() {
                if let Ok(english_dir) = service.find_dir(&entry.path(), ENGLISH_DIR) {
                    if let Ok(goals_path) = service.find_file(&english_dir, GOALS_FILE) {
                        let mission_dir = entry.file_name().to_string_lossy().to_string();
                        let custom_filename = format!("{}{}", mission_dir, PATH_TO_GOALS);
                        service.process_file(&goals_path, &custom_filename)?;
                    }
                }
            }
        }
        Ok(())
    }
}

impl FileProcessor for BooksProcessor {
    fn process(
        &self,
        base_path: &Path,
        service: &mut TranslationService,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let books_dir = service.find_dir(base_path, BOOKS_DIR)?;
        let scan_dir = service
            .find_dir(&books_dir, ENGLISH_DIR)
            .unwrap_or_else(|_| books_dir);

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
                service.process_file(&entry.path(), &filename)?;
            }
        }
        Ok(())
    }
}

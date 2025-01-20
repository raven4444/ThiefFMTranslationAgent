use crate::constants::*;
use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::{remove_polish_chars, wait_for_key_press};
use rusqlite::{Connection, OptionalExtension, Result};
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct TranslationCacheService {
    connection: Connection,
}

impl TranslationCacheService {
    pub fn new(directory_name: &str) -> Result<Self> {
        let db_path = Self::get_db_path(directory_name)?;

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?;
        }

        let connection = Connection::open(&db_path)?;

        connection.execute(SQL_CREATE_TABLE_TRANSLATIONS, [])?;

        Ok(TranslationCacheService { connection })
    }

    fn get_db_path(directory_name: &str) -> Result<PathBuf> {
        let path = Path::new(directory_name)
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(&*String::new())
            .to_string();
        let base_path = dirs::data_local_dir()
            .ok_or_else(|| rusqlite::Error::InvalidPath(CANNOT_FIND_APP_DATA_DIR.into()))?;

        let cache_dir = base_path.join(APP_NAME).join(CACHE_DIRECTORY).join(path);

        Ok(cache_dir.join(CACHE_DB_NAME))
    }

    pub fn insert(
        &mut self,
        filename: &str,
        original_path: &str,
        translated_path: &str,
        original_content: &str,
        translated_content: Option<&str>,
    ) -> Result<i64> {
        let tx = self.connection.transaction()?;

        tx.execute(
            SQL_INSERT_TRANSLATION,
            (
                filename,
                original_path,
                translated_path,
                original_content,
                translated_content,
            ),
        )?;

        let id = tx.last_insert_rowid();
        tx.commit()?;

        Ok(id)
    }

    pub fn find_by_original_path(&self, original_path: &str) -> Result<Option<i64>> {
        let mut stmt = self
            .connection
            .prepare(SQL_SELECT_ID_TRANSLATION_BY_ORIGINAL_PATH)?;

        let result = stmt
            .query_row([original_path], |row| row.get(0))
            .optional()?;

        Ok(result)
    }

    async fn translate_entry(
        _openai_client: &OpenAIClient,
        _prompt_service: &PromptService,
        _filename: &str,
        _content: &str,
    ) -> Result<String, Box<dyn Error>> {
        let proper_nouns = _prompt_service
            .get_prompt_by_type(PROMPT_TYPE_PROPER_NOUNS)
            .ok_or_else(|| {
                rusqlite::Error::InvalidParameterName(
                    format!("{}{}", PROMPT_NOT_FOUND, PROMPT_TYPE_PROPER_NOUNS).into(),
                )
            })?;

        let base_prompt_type = if _filename == OBJNAMES_FILE {
            PROMPT_TYPE_ITEMS
        } else if _filename.ends_with(GOALS_FILE) {
            PROMPT_TYPE_OBJECTIVES
        } else {
            PROMPT_TYPE_BOOKS
        };

        let base_prompt = _prompt_service
            .get_prompt_by_type(base_prompt_type)
            .ok_or_else(|| {
                rusqlite::Error::InvalidParameterName(
                    format!("{}{}", PROMPT_NOT_FOUND, base_prompt_type).into(),
                )
            })?;

        let combined_prompt = format!("{}\n{}", base_prompt.content, proper_nouns.content);
        let result = _openai_client
            .get_completions(&combined_prompt, _content)
            .await?;
        Ok(result)
    }

    pub async fn translate_cache(
        &mut self,
        openai_client: &OpenAIClient,
        prompt_service: &PromptService,
    ) -> std::result::Result<(), Box<dyn Error>> {
        let count: i64 = self
            .connection
            .query_row(SQL_COUNT_UNTRANSLATED, [], |row| row.get(0))?;
        println!(
            "{}{}{}{}",
            COLOR_YELLOW, FILES_TO_TRANSLATE, count, COLOR_RESET
        );
        wait_for_key_press()?;

        let mut stmt = self.connection.prepare(SQL_SELECT_UNTRANSLATED)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
            ))
        })?;
        let mut current = 0;

        for row in rows {
            let (id, filename, original_content) = row?;
            current += 1;
            print!("{}", LINE_START);
            print!(
                "{}{}{}{}{}{}",
                COLOR_YELLOW, TRANSLATING, current, FROM, count, COLOR_RESET
            );
            let translated_content =
                Self::translate_entry(openai_client, prompt_service, &filename, &original_content)
                    .await?;
            std::io::stdout().flush()?;
            self.connection.execute(
                SQL_UPDATE_TRANSLATED_CONTENT,
                [&translated_content, &id.to_string()],
            )?;
        }
        print!("{}", LINE_START);
        print!("{}{}{}", COLOR_GREEN, TRANSLATION_COMPLETE, COLOR_RESET);
        Ok(())
    }

    pub fn save_translations(&self) -> Result<()> {
        println!("{}{}{}", COLOR_GREEN, SAVING_TRANSLATIONS, COLOR_RESET);
        let mut stmt = self.connection.prepare(SQL_SELECT_TRANSLATIONS)?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
            ))
        })?;

        for row in rows {
            let (path, content) = row?;
            let processed_content = remove_polish_chars(&content);

            if let Some(parent) = Path::new(&path).parent() {
                fs::create_dir_all(parent).map_err(|e|
                    rusqlite::Error::InvalidPath(e.to_string().into())
                )?;
            }

            let mut file = fs::File::create(&path).map_err(|e|
                rusqlite::Error::InvalidPath(e.to_string().into())
            )?;

            file.write_all(processed_content.as_bytes()).map_err(|e|
                rusqlite::Error::InvalidPath(e.to_string().into())
            )?;
        }

        Ok(())
    }
}

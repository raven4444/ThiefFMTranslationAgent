use crate::constants::*;
use rusqlite::{Connection, OptionalExtension, Result};
use std::fs;
use std::path::{Path, PathBuf};

pub struct CacheService {
    connection: Connection,
}

impl CacheService {
    pub fn new(directory_name: &str) -> Result<Self> {
        let db_path = Self::get_db_path(directory_name)?;

        if let Some(parent) = db_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| rusqlite::Error::InvalidPath(e.to_string().into()))?;
        }

        let connection = Connection::open(&db_path)?;

        connection.execute(SQL_CREATE_TABLE_TRANSLATIONS, [])?;

        Ok(CacheService { connection })
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

    pub fn get_db_file_path(&self) -> Result<PathBuf> {
        Ok(self.connection.path().unwrap().into())
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
}

pub const SALT_SIZE_CONSTANT: usize = 32;
pub const NONCE_SIZE_CONSTANT: usize = 12;
pub const APP_NAME: &str = "ThiefFMTranslationAgent";
pub const STORAGE_FILE_NAME: &str = "secure.bin";
pub const REGISTRY_PATH: &str = "SOFTWARE\\Microsoft\\Cryptography";
pub const REGISTRY_KEY: &str = "MachineGuid";
pub const CACHE_DB_NAME: &str = "cache.sqlite";
pub const CACHE_DIRECTORY: &str = "cache";

//labels
pub const COLOR_GREEN: &str = "\x1b[32m"; // Green
pub const COLOR_RED: &str = "\x1b[31m"; // Red
pub const COLOR_YELLOW: &str = "\x1b[33m"; // Yellow
pub const COLOR_RESET: &str = "\x1b[0m";

pub const ENTER_API_KEY: &str = "Podaj klucz API do OpenAI: ";
pub const API_KEY_PREFIX: &str = "sk-";
pub const CENSORSHIP: &str = "****";
pub const INVALID_KEY_WARNING: &str = "Klucz jest nieprawidłowy.";
pub const VALID_KEY: &str = "Wszystko okej, klucz został zweryfikowany.";
pub const KEY_SAVED: &str = "Pomyślnie zaszyfrowano i zapisano klucz API.";
pub const KEY_RETRIEVED: &str = "Pomyślnie odszyfrowano klucz API.";
pub const KEY_RETRIEVAL_ERROR: &str = "Błąd podczas odczytu klucza API:";
pub const KEY_CORRUPTED: &str = "Zapisany klucz API jest niepoprawny.";
pub const KEY_NOT_FOUND: &str = "Nie znaleziono klucza API.";
pub const KEY_FOUND: &str = "Znaleziono zapisany klucz API.";
pub const USE_KEY: &str = "Używam klucza:";
pub const CONTINUE: &str = "Naciśnij enter aby kontynuować...";
pub const CANNOT_FIND_APP_DATA_DIR: &str = "Nie można znaleźć katalogu %AppData%/Local";
pub const REUSE_KEY: &str = "Czy chcesz użyć zapisanego klucza API? (T/n): ";
pub const UPDATE_APP: &str = "Czy chcesz otworzyć stronę aby pobrać nową wersję? (T/n): ";
pub const YES: &str = "t";
pub const OPENAI_API_ERROR: &str = "API OpenAI zwróciło błąd: ";
pub const NEW_VERSION: &str = "Dostępna jest nowa wersja aplikacji - ";
pub const PATH_TO_FM: &str =  "Podaj ścieżkę do fanmisji zainstalowanej przez AngelLoader, domyślnie jest to folder /FMs/nazwa_misji w katologu gry: ";
pub const MISSING_DIRECTORY: &str = "Katalog nie istnieje!";
pub const INVALID_PATH: &str = "Podana ścieżka jest nieprawidłowa!";
pub const MISSING_FM_FILES: &str = "Nie znaleziono plików misji!";
pub const MISSION_DIRECTORY: &str = "Katalog wybranej fanmisji to: ";
pub const ISSUE_WITH_CACHE: &str =  "Wystąpił problem z keszem, aplikacja zostanie zamknięta. W razie utrzymujących się problemów szukaj pomocy na githubie/forum.";
pub const AVAILABLE_PROMPTS: &str = "Dostępne prompty:";
pub const CANNOT_DECODE_CHARACTERS: &str = "Nie można odczytać znaków korzystając z kodowania: ";
pub const DIRECTORY_NOT_FOUND: &str = "Nie znaleziono katalogu: ";
pub const FILE_NOT_FOUND: &str = "Nie znaleziono pliku: ";

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";
pub const OPENAI_MODEL: &str = "gpt-4o";
pub const PROMPTS_FILE: &str = "prompts.json";
pub const FILE_SERVER_BASE_URL: &str = "https://raw.githubusercontent.com/raven4444/ThiefFMTranslationAgent/refs/heads/master/assets/prompts/";
pub const RELEASES_URL: &str =
    "https://api.github.com/repos/raven4444/ThiefFMTranslationAgent/releases/latest";
pub const VERSION_PREFIX: &str = "v";
pub const MIS_EXTENSION: &str = "mis";
pub const BOM: &str = "\u{FEFF}";
pub const STRINGS_DIR: &str = "strings";
pub const ENGLISH_DIR: &str = "english";
pub const POLISH_DIR: &str = "polish";
pub const OBJNAMES_FILE: &str = "objnames.str";
pub const INTRFACE_DIR: &str = "intrface";
pub const BOOKS_DIR: &str = "books";
pub const MISS_DIR: &str = "miss";
pub const GOALS_FILE: &str = "goals.str";
pub const PATH_TO_GOALS: &str = "/english/goals.str";
pub const STR_EXTENSION: &str = "str";

pub const SQL_CREATE_TABLE_TRANSLATIONS: &str = "CREATE TABLE IF NOT EXISTS translations (
    id INTEGER PRIMARY KEY,
    filename TEXT NOT NULL,
    original_path TEXT NOT NULL,
    translated_path TEXT NOT NULL,
    original_content TEXT NOT NULL,
    translated_content TEXT
)";
pub const SQL_INSERT_TRANSLATION: &str = "INSERT INTO translations (filename, original_path, translated_path, original_content, translated_content) VALUES (?1, ?2, ?3, ?4, ?5)";
pub const SQL_SELECT_ID_TRANSLATION_BY_ORIGINAL_PATH: &str = "SELECT id FROM translations WHERE original_path = ?1";
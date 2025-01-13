pub const SALT_SIZE_CONSTANT: usize = 32;
pub const NONCE_SIZE_CONSTANT: usize = 12;
pub const APP_NAME: &str = "ThiefFMTranslationAgent";
pub const STORAGE_FILE_NAME: &str = "secure.bin";
pub const REGISTRY_PATH: &str = "SOFTWARE\\Microsoft\\Cryptography";
pub const REGISTRY_KEY: &str = "MachineGuid";

//labels
pub const COLOR_GREEN: &str = "\x1b[32m"; // Green
pub const COLOR_RED: &str = "\x1b[31m"; // Red
pub const COLOR_YELLOW: &str = "\x1b[33m"; // Yellow
pub const COLOR_RESET: &str = "\x1b[0m";

pub const ENTER_API_KEY: &str = "Podaj klucz API do OpenAI: ";
pub const API_KEY_PREFIX: &str = "sk-";
pub const CENSORSHIP: &str = "****";
pub const INVALID_KEY_WARNING: &str =
    "Klucz jest nieprawidłowy.";
pub const VALID_KEY: &str = "Wszystko okej, klucz został zweryfikowany.";
pub const KEY_SAVED: &str = "Pomyślnie zaszyfrowano i zapisano klucz API.";
pub const KEY_RETRIEVED: &str = "Pomyślnie odszyfrowano klucz API.";
pub const KEY_RETRIEVAL_ERROR: &str = "Błąd podczas odczytu klucza API:";
pub const KEY_CORRUPTED: &str = "Zapisany klucz API jest niepoprawny.";
pub const KEY_NOT_FOUND: &str = "Nie znaleziono klucza API.";
pub const KEY_FOUND: &str = "Znaleziono zapisany klucz API.";
pub const USE_KEY: &str = "Używam klucza:";
pub const CONTINUE: &str = "Naciśnij dowolny klawisz aby kontynuować...";
pub const CANNOT_FIND_APP_DATA_DIR: &str = "Nie można znaleźć katalogu %AppData%/Local";
pub const REUSE_KEY: &str = "Czy chcesz użyć zapisanego klucza API? (T/n): ";
pub const YES: &str = "t";

pub const OPENAI_API_URL: &str = "https://api.openai.com/v1";
pub const OPENAI_MODEL: &str = "gpt-4o";
pub const OPENAI_API_ERROR: &str = "API OpenAI zwróciło błąd: ";

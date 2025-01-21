# ThiefFMTranslationAgent - TFMTA
![GitHub Release](https://img.shields.io/github/v/release/raven4444/ThiefFMTranslationAgent)

> **Warning**
>
> Presently, app supports translation to Polish language only.

> **Uwaga**
>
> Ze wezględu na ograniczenia wbudowanych czcionek agent nie wspiera polskich liter w tłumaczeniach. Wszystkie polskie znaki zostaną zastąpione przez ich odpowiedniki bez ogonków.

## Opis

TFMTA wykorzystuje modele OpenAI do automatycznego tłumaczenia treści z fanmisji do gry "Thief Gold", "Thief: The Dark Project", "Thief: The Metal Age".

## Obecny zakres tłumaczeń

- nazwy przedmiotów,
- zawartość ksiąg,
- cele misji.

## Wymagania

- Thief The Metal Age/Thief Gold/Thief The Dark Project z zainstalowanym TFixem,
- AngelLoader,
- Klucz API OpenAI wraz z budżetem i dostępem do modelu gpt4o.

## Instalacja

### Wersja dla użytkownika końcowego
1. Pobierz najnowszą wersję z [releases](https://github.com/raven4444/ThiefFMTranslationAgent/releases/latest).
2. Uruchom aplikację.
3. Aplikacja poprowadzi Cię przez proces konfiguracji i tłumaczenia.

### Wersja dla programistów - wymagany devkit [Rust](https://www.rust-lang.org/tools/install)

1. Sklonuj repozytorium:
   ```bash
   git clone https://github.com/raven4444/ThiefFMTranslationAgent.git
   cd ThiefFMTranslationAgent
2. Skompiluj aplikację:
   ```bash
   cargo build --release
3. Uruchom aplikację:
   ```bash
   cd target/release
    ./ThiefFMTranslationAgent.exe
   
4. Aplikacja poprowadzi Cię przez proces konfiguracji i tłumaczenia.

## License
This tool is licensed under the GNU General Public License v3.0.
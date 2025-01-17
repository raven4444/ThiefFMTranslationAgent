use crate::openai_client::OpenAIClient;
use crate::prompt_service::PromptService;
use crate::utils::get_fm_directory_path;

pub struct TranslationService {
    openai_client: OpenAIClient,
    prompt_service: PromptService,
    path: String
}

impl TranslationService {
    pub fn new(openai_client: OpenAIClient, prompt_service: PromptService) -> Self {
        TranslationService {
            openai_client,
            prompt_service,
            path: String::new()
        }
    }

    pub fn run(&mut self) -> Result<String, Box<dyn std::error::Error>> {
        self.path = get_fm_directory_path()?;
        //todo
        Ok("Processing completed".to_string())
    }
}

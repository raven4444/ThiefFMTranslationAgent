use openai::models::Model;
use openai::Credentials;
use std::mem::replace;

pub struct OpenAIClient {
    credentials: Credentials,
}

impl Drop for OpenAIClient {
    fn drop(&mut self) {
        let empty_creds = Credentials::new(String::new(), String::new());
        let _ = replace(&mut self.credentials, empty_creds);
    }
}

impl OpenAIClient {
    pub fn new(api_key: String) -> Self {
        OpenAIClient {
            credentials: Credentials::new(api_key, "https://api.openai.com/v1"),
        }
    }

    pub async fn verify_key(&self) -> bool {
        match Model::fetch("gpt-4o", self.credentials.clone()).await {
            Ok(model) => !model.id.is_empty(),
            Err(e) => {
                eprintln!("API OpenAI zwróciło błąd: {}", e.message);
                false
            }
        }
    }
}

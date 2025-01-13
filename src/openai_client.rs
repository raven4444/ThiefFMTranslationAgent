use crate::constants::*;
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
            credentials: Credentials::new(api_key, OPENAI_API_URL),
        }
    }

    pub async fn verify_key(&self) -> bool {
        match Model::fetch(OPENAI_MODEL, self.credentials.clone()).await {
            Ok(model) => !model.id.is_empty(),
            Err(e) => {
                eprintln!("{}{}", OPENAI_API_ERROR, e.message);
                false
            }
        }
    }
}

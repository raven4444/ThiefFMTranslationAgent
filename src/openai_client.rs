use crate::constants::*;
use openai::chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole};
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

    pub async fn get_completions(
        &self,
        system_prompt: &str,
        user_prompt: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let messages = vec![
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::System,
                content: Some(system_prompt.to_string()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: vec![],
            },
            ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(user_prompt.to_string()),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: vec![],
            },
        ];
        let chat_completion = ChatCompletion::builder(OPENAI_MODEL, messages.clone())
            .credentials(self.credentials.clone())
            .create()
            .await
            .unwrap();
        Ok(chat_completion
            .choices
            .first()
            .unwrap()
            .message
            .clone()
            .content
            .unwrap())
    }
}

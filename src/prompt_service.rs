use crate::constants::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCollection {
    prompts: Vec<Prompt>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    content: String,
    version: String,
    #[serde(rename = "type")]
    prompt_type: String,
}

pub struct PromptService {
    prompts: Vec<Prompt>,
}

impl PromptService {
    pub async fn initialize() -> Result<Self> {
        let client = reqwest::Client::new();

        let prompts = client
            .get(PROMPTS_URL)
            .send()
            .await?
            .json::<PromptCollection>()
            .await?
            .prompts;

        Ok(Self { prompts })
    }

    pub fn get_prompt_by_type(&self, prompt_type: &str) -> Option<&Prompt> {
        self.prompts.iter().find(|p| p.prompt_type == prompt_type)
    }

    pub fn print_prompts_overview(&self) {
        println!("{}{}", COLOR_GREEN, AVAILABLE_PROMPTS);
        for prompt in &self.prompts {
            println!("{} ->> {}", prompt.prompt_type, prompt.version);
        }
    }
}

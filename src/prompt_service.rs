use crate::constants::*;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptCollection {
    prompts: Vec<PromptFileInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PromptFileInfo {
    file: String,
    version: String,
    #[serde(rename = "type")]
    prompt_type: String,
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

        let prompt_infos = client
            .get(format!("{}{}", FILE_SERVER_BASE_URL, PROMPTS_FILE))
            .send()
            .await?
            .json::<PromptCollection>()
            .await?
            .prompts;

        let mut prompts = Vec::new();
        for info in prompt_infos {
            let file_url = format!("{}{}", FILE_SERVER_BASE_URL, info.file);

            let content = client
                .get(&file_url)
                .send()
                .await?
                .text()
                .await?;

            prompts.push(Prompt {
                content,
                version: info.version,
                prompt_type: info.prompt_type,
            });
        }

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

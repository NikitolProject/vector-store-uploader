use async_openai::{
    config::OpenAIConfig,
    types::{CreateEmbeddingRequest, EmbeddingInput},
    Client as OpenAIClient,
};
use async_trait::async_trait;
use super::EmbeddingCreator;

pub struct OpenAIEmbeddingCreator {
    client: OpenAIClient<OpenAIConfig>,
}

impl OpenAIEmbeddingCreator {
    pub fn new(api_key: String) -> Self {
        let config = OpenAIConfig::new().with_api_key(api_key);
        let client = OpenAIClient::with_config(config);
        Self { client }
    }
}

#[async_trait]
impl EmbeddingCreator for OpenAIEmbeddingCreator {
    async fn create_embedding(&self, text: String) -> Result<Vec<f32>, String> {
        let mut retries = 3;
        
        while retries > 0 {
            let request = CreateEmbeddingRequest {
                model: "text-embedding-3-large".into(),
                input: EmbeddingInput::String(text.clone()),
                encoding_format: None,
                dimensions: None,
                user: None,
            };

            match self.client.embeddings().create(request).await {
                Ok(response) => {
                    return Ok(response.data[0].embedding.clone());
                }
                Err(e) => {
                    println!("Ошибка при создании эмбеддинга: {}", e);
                    retries -= 1;
                    if retries == 0 {
                        return Err(format!("Не удалось создать эмбеддинг после 3 попыток: {}", e));
                    }
                    println!("Ожидание 2 секунды перед повторной попыткой...");
                    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                }
            }
        }

        Err("Произошла непредвиденная ошибка".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_embedding() {
        let creator = OpenAIEmbeddingCreator::new("test-key".to_string());
        let result = creator.create_embedding("тестовый текст".to_string()).await;
        assert!(result.is_err()); // Должен вернуть ошибку с недействительным ключом API
    }
}

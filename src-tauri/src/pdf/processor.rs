use super::PdfProcessor;
use crate::embeddings::{EmbeddingCreator, OpenAIEmbeddingCreator};
use crate::storage::{VectorStorage, PineconeStorage};
use crate::settings::Settings;
use std::path::Path;
use futures::future::join_all;

pub struct DefaultPdfProcessor {
    embedding_creator: OpenAIEmbeddingCreator,
    vector_storage: PineconeStorage,
}

impl DefaultPdfProcessor {
    pub async fn new(settings: &Settings) -> Result<Self, String> {
        let embedding_creator = OpenAIEmbeddingCreator::new(settings.openai_api_key.clone());
        let vector_storage = PineconeStorage::new(
            settings.pinecone_api_key.clone(),
            settings.pinecone_index_host.clone(),
            settings.pinecone_namespace.clone(),
        ).await?;

        Ok(Self {
            embedding_creator,
            vector_storage,
        })
    }
}

#[async_trait::async_trait]
impl PdfProcessor for DefaultPdfProcessor {
    async fn process_file(&mut self, file_path: &Path) -> Result<String, String> {
        // Извлекаем текст из PDF
        let text = pdf_extract::extract_text(file_path)
            .map_err(|e| format!("Ошибка извлечения текста из PDF: {}", e))?;

        println!(
            "Успешно извлечен текст из PDF, длина: {} символов",
            text.len()
        );

        // Разбиваем текст на чанки и обрабатываем
        self.process_text(text).await
    }

    async fn process_text(&mut self, text: String) -> Result<String, String> {
        const CHUNK_SIZE: usize = 8000;

        // Разбиваем текст на чанки по символам
        let chunks: Vec<String> = text
            .chars()
            .collect::<Vec<char>>()
            .chunks(CHUNK_SIZE)
            .map(|chunk| chunk.iter().collect::<String>())
            .collect();

        println!("Текст разбит на {} чанков", chunks.len());

        // Очищаем существующие векторы
        self.vector_storage.clear().await?;

        let embedding_creator = &self.embedding_creator;
        let futures: Vec<_> = chunks
            .iter()
            .enumerate()
            .map(|(i, chunk)| async move {
                let embedding = embedding_creator.create_embedding(chunk.clone()).await?;
                Ok::<(usize, Vec<f32>, String), String>((i, embedding, chunk.clone()))
            })
            .collect();

        let results = join_all(futures).await;
        
        for result in results {
            let (i, embedding, chunk) = result?;
            self.vector_storage
                .store_vector(format!("chunk_{}", i), embedding, chunk)
                .await?;
            println!("Обработан чанк {}/{}", i + 1, chunks.len());
        }

        Ok(format!(
            "Успешно обработано и загружено {} чанков",
            chunks.len()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::settings::Settings;

    #[tokio::test]
    async fn test_process_text() {
        let settings = Settings {
            openai_api_key: "test_openai_key".to_string(),
            pinecone_api_key: "test_pinecone_key".to_string(),
            pinecone_index_host: "test_host".to_string(),
            pinecone_namespace: "test_namespace".to_string(),
        };

        let processor = DefaultPdfProcessor::new(&settings).await;
        assert!(processor.is_err()); // Должен вернуть ошибку с тестовыми ключами
    }
}

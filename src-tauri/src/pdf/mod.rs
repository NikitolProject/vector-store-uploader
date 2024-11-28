mod processor;
pub use processor::*;

use std::path::Path;

#[async_trait::async_trait]
pub trait PdfProcessor {
    async fn process_file(&mut self, file_path: &Path) -> Result<String, String>;
    async fn process_text(&mut self, text: String) -> Result<String, String>;
}

#[cfg(test)]
pub(crate) mod test_utils {
    use async_trait::async_trait;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use crate::embeddings::EmbeddingCreator;
    use crate::storage::VectorStorage;

    // Мок для EmbeddingCreator
    pub struct MockEmbeddingCreator;

    #[async_trait]
    impl EmbeddingCreator for MockEmbeddingCreator {
        async fn create_embedding(&self, _text: String) -> Result<Vec<f32>, String> {
            Ok(vec![0.1, 0.2, 0.3])
        }
    }

    // Мок для VectorStorage
    pub struct MockVectorStorage {
        vectors: Arc<Mutex<Vec<(String, Vec<f32>, String)>>>,
    }

    impl MockVectorStorage {
        pub fn new() -> Self {
            Self {
                vectors: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl VectorStorage for MockVectorStorage {
        async fn store_vector(
            &mut self,
            id: String,
            vector: Vec<f32>,
            text: String,
        ) -> Result<(), String> {
            self.vectors.lock().await.push((id, vector, text));
            Ok(())
        }

        async fn clear(&mut self) -> Result<(), String> {
            self.vectors.lock().await.clear();
            Ok(())
        }
    }
}

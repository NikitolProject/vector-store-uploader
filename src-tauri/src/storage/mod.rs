mod pinecone;
pub use pinecone::*;

use async_trait::async_trait;

#[async_trait]
pub trait VectorStorage: Send + Sync {
    async fn store_vector(
        &mut self,
        id: String,
        vector: Vec<f32>,
        text: String,
    ) -> Result<(), String>;
    
    async fn clear(&mut self) -> Result<(), String>;
}

#[cfg(test)]
pub(crate) mod test_utils {
    use super::*;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub struct MockVectorStorage {
        vectors: Arc<Mutex<Vec<(String, Vec<f32>, String)>>>,
    }

    impl MockVectorStorage {
        pub fn new() -> Self {
            Self {
                vectors: Arc::new(Mutex::new(Vec::new())),
            }
        }

        pub async fn get_vectors(&self) -> Vec<(String, Vec<f32>, String)> {
            self.vectors.lock().await.clone()
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

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn test_mock_vector_storage() {
            let mut storage = MockVectorStorage::new();
            
            // Тест сохранения вектора
            assert!(storage
                .store_vector(
                    "test_id".to_string(),
                    vec![0.1, 0.2, 0.3],
                    "test text".to_string()
                )
                .await
                .is_ok());

            // Проверяем сохраненные данные
            let vectors = storage.get_vectors().await;
            assert_eq!(vectors.len(), 1);
            assert_eq!(vectors[0].0, "test_id");
            assert_eq!(vectors[0].1, vec![0.1, 0.2, 0.3]);
            assert_eq!(vectors[0].2, "test text");

            // Тест очистки
            assert!(storage.clear().await.is_ok());
            assert_eq!(storage.get_vectors().await.len(), 0);
        }
    }
}

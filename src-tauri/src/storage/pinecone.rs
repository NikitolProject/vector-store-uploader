use super::VectorStorage;
use async_trait::async_trait;
use pinecone_sdk::{
    models::{Kind, Metadata, Namespace, Value, Vector},
    pinecone::{PineconeClientConfig, data::Index},
};
use std::collections::BTreeMap;

pub struct PineconeStorage {
    index: Index,
    namespace: Namespace,
}

impl PineconeStorage {
    pub async fn new(
        api_key: String,
        index_host: String,
        namespace: String,
    ) -> Result<Self, String> {
        let client_config = PineconeClientConfig {
            api_key: Some(api_key),
            ..Default::default()
        };

        let client = client_config
            .client()
            .map_err(|e| format!("Ошибка создания клиента Pinecone: {}", e))?;

        let index = client
            .index(&index_host)
            .await
            .map_err(|e| format!("Ошибка подключения к индексу Pinecone: {}", e))?;

        Ok(Self {
            index,
            namespace: Namespace::from(namespace),
        })
    }
}

#[async_trait]
impl VectorStorage for PineconeStorage {
    async fn store_vector(
        &mut self,
        id: String,
        vector: Vec<f32>,
        text: String,
    ) -> Result<(), String> {
        let vector = Vector {
            id,
            values: vector,
            sparse_values: None,
            metadata: Some(Metadata {
                fields: BTreeMap::from([(
                    "text".to_string(),
                    Value {
                        kind: Some(Kind::StringValue(text)),
                    },
                )]),
            }),
        };

        self.index
            .upsert(&[vector], &self.namespace)
            .await
            .map(|_| ())
            .map_err(|e| format!("Ошибка сохранения вектора: {}", e))
    }

    async fn clear(&mut self) -> Result<(), String> {
        self.index
            .delete_all(&self.namespace)
            .await
            .map_err(|e| format!("Ошибка очистки векторов: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    #[ignore]
    async fn test_pinecone_storage() {
        let storage = PineconeStorage::new(
            "test_api_key".to_string(),
            "test_index_host".to_string(),
            "test_namespace".to_string(),
        )
        .await;
        assert!(storage.is_ok());
    }
}

mod creator;
pub use creator::*;
use async_trait::async_trait;

#[async_trait]
pub trait EmbeddingCreator {
    async fn create_embedding(&self, text: String) -> Result<Vec<f32>, String>;
}

use super::EmbeddingFunction;
use crate::commons::{Embedding, Result};
pub use rust_bert::pipelines::sentence_embeddings::*;

impl EmbeddingFunction for SentenceEmbeddingsModel {
    fn embed(&self, docs: &[&str]) -> Result<Vec<Embedding>> {
        Ok(self.encode(docs)?)
    }
}

#[cfg(test)]
mod tests {
    use crate::collection::CollectionEntries;
    use crate::ChromaClient;
    use super::*;

    #[tokio::test]
    async fn test_sbert_embeddings() {

        let client = ChromaClient::new(Default::default());
        let collection = client
            .get_or_create_collection("sbert-test-collection", None)
            .await
            .unwrap();

        let sbert_embedding =
            SentenceEmbeddingsBuilder::remote(SentenceEmbeddingsModelType::AllMiniLmL6V2)
                .create_model()
                .unwrap();

        let docs = vec![
            "Once upon a time there was a frog",
            "Once upon a time there was a cow",
            "Once upon a time there was a wolverine",
        ];

        let collection_entries = CollectionEntries {
            ids: vec!["test1", "test2", "test3"],
            metadatas: None,
            documents: Some(docs),
            embeddings: None,
        };

        collection
            .upsert(collection_entries, Some(Box::new(sbert_embedding)))
            .await
            .unwrap();
    }
}

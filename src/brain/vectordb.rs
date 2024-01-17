use qdrant_client::{
    prelude::QdrantClient,
    qdrant::{SearchPoints, SearchResponse},
};

use crate::brain::config;

pub fn build_client() -> QdrantClient {
    let vectordb_client = QdrantClient::from_url(config::VECTOR_DATABASE_URL)
        .with_api_key(config::VECTOR_DATABASE_API_KEY)
        .build()
        .unwrap();

    return vectordb_client;
}

pub async fn find_relevant_information(
    vector: &Vec<f32>,
    client: &QdrantClient,
) -> Result<Vec<String>, anyhow::Error> {
    let search_result = client
        .search_points(&SearchPoints {
            collection_name: config::VECTOR_COLLECTION_SELLPY_BRAIN.into(),
            vector: vector.clone(),
            limit: 10,
            with_payload: Some(true.into()),
            ..Default::default()
        })
        .await?;

    return Ok(extract_payload(&search_result, "content"));
}

fn extract_payload(search_result: &SearchResponse, key: &'static str) -> Vec<String> {
    return search_result
        .result
        .clone()
        .into_iter()
        .map(|point| match point.payload.get(key) {
            Some(content) => {
                return content.as_str().unwrap().to_string();
            }
            None => {
                return String::new();
            }
        })
        .collect();
}

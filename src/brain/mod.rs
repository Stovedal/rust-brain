use async_openai::{config::OpenAIConfig, Client};
use qdrant_client::prelude::QdrantClient;

mod config;
mod prompting;

pub mod llm;
pub mod vectordb;

pub async fn ask(
    llm_client: &Client<OpenAIConfig>,
    vectordb_client: &QdrantClient,
    question: &str,
) -> Result<String, anyhow::Error> {
    let question_vector = llm::create_embedding(llm_client, question).await?;

    let information =
        vectordb::find_relevant_information(&question_vector, vectordb_client).await?;

    let messages = vec![
        llm::build_user_message(prompting::build_setup_prompt()),
        llm::build_user_message(prompting::build_answer_question_prompt(
            question,
            information,
        )),
    ];

    let answer = llm::chat(llm_client, messages).await?;

    return Ok(answer);
}

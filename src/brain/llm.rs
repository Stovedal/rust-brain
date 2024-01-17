use async_openai::{
    config::OpenAIConfig,
    types::{
        ChatCompletionRequestMessage, CreateChatCompletionRequestArgs, CreateEmbeddingRequestArgs,
        Role,
    },
    Client,
};

use crate::brain::config;

pub fn build_client() -> Client<OpenAIConfig> {
    let config: OpenAIConfig = OpenAIConfig::new().with_api_key(config::OPEN_AI_KEY);

    let client: Client<OpenAIConfig> = Client::with_config(config);

    return client;
}

pub fn build_user_message(message: String) -> ChatCompletionRequestMessage {
    return ChatCompletionRequestMessage {
        role: Role::User,
        content: Some(message.to_string()),
        ..Default::default()
    };
}

pub async fn chat(
    client: &Client<OpenAIConfig>,
    messages: Vec<ChatCompletionRequestMessage>,
) -> Result<String, anyhow::Error> {
    let request = CreateChatCompletionRequestArgs::default()
        .model("gpt-4")
        .messages(messages)
        .build()?;

    let response = client.chat().create(request).await.unwrap();

    let answer = response.choices[0].message.content.clone();

    match answer {
        Some(answer) => {
            return Ok(answer);
        }
        None => {
            return Err(anyhow::Error::msg("No response from OpenAI"));
        }
    }
}

pub async fn create_embedding(
    client: &Client<OpenAIConfig>,
    text: &str,
) -> Result<Vec<f32>, anyhow::Error> {
    let request = CreateEmbeddingRequestArgs::default()
        .model("text-embedding-ada-002")
        .input(text)
        .build()
        .expect("Not cool");

    let response = client.embeddings().create(request).await?;

    return Ok(response.data[0].embedding.clone());
}

use actix_web::{post, web, HttpResponse};
use serde::Deserialize;

use crate::llm::openai::OpenAI;
use crate::llm::openrouter::OpenRouter;

#[derive(Deserialize)]
pub struct AgentRequest {
    pub instruction: String,
    pub code: String,
}

#[post("/agent")]
pub async fn run_agent(
    data: web::Json<AgentRequest>,
    llm: web::Data<OpenRouter>,
) -> HttpResponse {

    let prompt = format!(
        "Instruction: {}\n\nCode:\n{}",
        data.instruction,
        data.code
    );

    match llm.generate(&prompt).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
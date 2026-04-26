use actix_web::{post, web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use crate::llm::openai::OpenAI;
use crate::llm::openrouter::OpenRouter;
use sqlx::PgPool;
use crate::memory::embedding::EmbeddingService;
use crate::memory::search::search_similar;


#[derive(Deserialize)]
pub struct AgentRequest {
    pub instruction: String,
    pub code: String,
    pub project_id: Uuid,
}


pub async fn run_agent(
    data: web::Json<AgentRequest>,
    llm: web::Data<OpenRouter>,
    pool: web::Data<PgPool>,                 // 🔥 NEW
    embedding_service: web::Data<EmbeddingService>, // 🔥 NEW
) -> HttpResponse {

    println!("hjrnjtrhghtggg");

    // 1️⃣ Generate embedding from code
    let embedding = match embedding_service.generate(&data.code).await {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Embedding error: {:?}", e);
            return HttpResponse::InternalServerError().body("Embedding failed");
        }
    };

    // 2️⃣ Search similar files
    let context_files = match search_similar(
        &pool,
        data.project_id,
        embedding
    ).await {
        Ok(files) => files,
        Err(e) => {
            eprintln!("Search error: {:?}", e);
            vec![]
        }
    };

    // 3️⃣ Build context
    let context = context_files.join("\n");

    // 4️⃣ Build smarter prompt
    let prompt = format!(
        "You are a senior software engineer.\n\n\
        Project Context:\n{}\n\n\
        Instruction: {}\n\n\
        Code:\n{}",
        context,
        data.instruction,
        data.code
    );

    // 5️⃣ Call LLM
    match llm.generate(&prompt).await {
        Ok(result) => HttpResponse::Ok().json(result),
        Err(e) => {
            eprintln!("LLM error: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
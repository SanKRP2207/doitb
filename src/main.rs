use actix_web::{App, HttpServer, web};
use llm::openrouter::OpenRouter;
use sqlx::PgPool;

use crate::memory::embedding::EmbeddingService;
mod llm;
mod memory;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // ✅ MUST be inside function
    dotenv::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("🔗 Connecting to DB at {}", database_url);

    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to DB");

    let api_key = std::env::var("OPENROUTER_API_KEY").expect("OPENROUTER_API_KEY not set");

    // 🤖 OpenRouter client
    let openrouter = OpenRouter {
        api_key: api_key.clone(),
    };

    // TODO: Initialize your embedding_service here
    // Replace the following line with actual initialization as needed
    let embedding_service = EmbeddingService {
        api_key: api_key.clone(),
    };

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(openrouter.clone()))
            .app_data(web::Data::new(embedding_service.clone()))
            // .service(routes::agent::run_agent)
            .route("/agent", web::post().to(routes::agent::run_agent))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

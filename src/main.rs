use actix_web::{App, HttpServer, web};
use llm::openrouter::OpenRouter;

mod routes;
mod llm;
mod memory;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // ✅ MUST be inside function
    dotenv::dotenv().ok();

    let openrouter = OpenRouter {
        api_key: std::env::var("OPENROUTER_API_KEY")
            .expect("OPENROUTER_API_KEY not set"),
    };

    println!("🚀 Server running at http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(openrouter.clone()))
            .service(routes::agent::run_agent)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
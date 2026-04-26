use crate::memory::embedding::EmbeddingService;
use crate::memory::vector_search::vector_search;
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
#[async_trait]
pub trait LlmProvider {
    async fn generate(&self, system: &str, prompt: &str) -> anyhow::Result<String>;
}

pub struct AgentOrchestrator<L: LlmProvider> {
    llm: L,
    db: PgPool,
    embedding_service: EmbeddingService,
}

impl<L: LlmProvider> AgentOrchestrator<L> {
    pub async fn handle_request(
        &self,
        user_input: String,
        project_id: Uuid,
    ) -> anyhow::Result<String> {
        // 1. Retrieve relevant files
        let context = vector_search(project_id, &user_input, &self.db, &self.embedding_service).await?;
        // let context = "No context yet".to_string();

        // 2. Planning step
        let plan = self
            .llm
            .generate("You are a senior architect planner.", &user_input)
            .await?;

        // 3. Coding step
        let code_output = self
            .llm
            .generate(
                "You are an expert Rust/Node developer.",
                &format!("Context: {}\nPlan: {}\nTask: {}", context, plan, user_input),
            )
            .await?;

        // 4. Optional: Execute in sandbox
        // run_tests(code_output).await?;

        Ok(code_output)
    }
}

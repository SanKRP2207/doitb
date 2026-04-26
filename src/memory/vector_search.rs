use anyhow::Result;
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::memory::embedding::EmbeddingService;

pub async fn vector_search(
    project_id: Uuid,
    user_input: &str,
    db: &PgPool,
    embedding_service: &EmbeddingService,
) -> Result<String> {
    // 1. Generate embedding from user input
    let embedding = embedding_service.generate(user_input).await?;

    // 2. Convert to pgvector format
    let embedding_str = format!(
        "[{}]",
        embedding.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(",")
    );

    // 3. Query DB
    let rows = sqlx::query(
        r#"
        SELECT summary
        FROM file_embeddings
        WHERE project_id = $1
        ORDER BY embedding <-> $2::vector
        LIMIT 5
        "#
    )
    .bind(project_id)
    .bind(embedding_str)
    .fetch_all(db)
    .await?;

    // 4. Convert to string
    let context = rows
        .iter()
        .map(|row| row.get::<String, _>("summary"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(context)
}
use sqlx::PgPool;
use uuid::Uuid;

pub async fn store_embedding(
    pool: &PgPool,
    project_id: Uuid,
    file_path: String,
    embedding: Vec<f32>,
) -> anyhow::Result<()> {

    sqlx::query(
        "INSERT INTO file_embeddings (id, project_id, file_path, embedding)
         VALUES ($1, $2, $3, $4)"
    )
    .bind(Uuid::new_v4())
    .bind(project_id)
    .bind(file_path)
    .bind(embedding)
    .execute(pool)
    .await?;

    Ok(())
}
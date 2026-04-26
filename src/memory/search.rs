use sqlx::{PgPool, Row};
use uuid::Uuid;

pub async fn search_similar(
    db: &PgPool,
    project_id: Uuid,
    embedding: Vec<f32>,
) -> anyhow::Result<Vec<String>> {
    let rows = sqlx::query(
        r#"
        SELECT file_path
        FROM file_embeddings
        WHERE project_id = $1
        ORDER BY embedding <-> $2
        LIMIT 5
        "#,
    )
    .bind(project_id)
    .bind(&embedding)
    .fetch_all(db)
    .await?;

    let result = rows
        .into_iter()
        .map(|row| row.get::<String, _>("file_path"))
        .collect();

    Ok(result)
}

use sqlx::Pool;
use uuid::Uuid;

pub async fn search_similar(
    db: &Pool,
    project_id: Uuid,
    embedding: Vec<f32>,
) -> anyhow::Result<Vec<String>> {

    let rows = sqlx::query!(
        r#"
        SELECT file_path
        FROM file_embeddings
        WHERE project_id = $1
        ORDER BY embedding <-> $2
        LIMIT 5
        "#,
        project_id,
        &embedding
    )
    .fetch_all(db)
    .await?;

    Ok(rows.into_iter().map(|r| r.file_path).collect())
}
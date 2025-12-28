use axum::{Json, extract::State, response::IntoResponse};
use sqlx::{FromRow, SqlitePool};
pub async fn create_table_members(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        CREATE TABLE IF NOT EXISTS members (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nome TEXT NOT NULL,
            idade INTEGER NOT NULL,
            email TEXT NOT NULL,
            genero TEXT NOT NULL,
            celular TEXT NOT NULL,
            estado_civil TEXT NOT NULL,
            endereco TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn insert_members(pool: &SqlitePool, membro: &Member) -> Result<(), sqlx::Error> {
    create_table_members(pool).await?;
    let member = membro;
    sqlx::query!(
        r#"
           INSERT INTO members
           (nome, idade, email, genero, celular,estado_civil,endereco)
           VALUES (?, ?, ?, ?, ?, ?,?)
           "#,
        member.nome,
        member.idade,
        member.email,
        member.genero,
        member.celular,
        member.estado_civil,
        member.endereco,
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn index(State(pool): State<SqlitePool>) -> impl IntoResponse {
    if let Err(e) = create_table_members(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }

    "hello world".into_response()
}
pub async fn add_member(
    State(pool): State<SqlitePool>,
    Json(membro): Json<Member>,
) -> impl IntoResponse {
    if let Err(e) = create_table_members(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    match insert_members(&pool, &membro).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Member {
    id: Option<i32>,
    nome: String,
    idade: u32,
    email: String,
    genero: String,
    celular: String,
    estado_civil: String,
    endereco: String,
}

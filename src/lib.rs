use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sqlx::{FromRow, SqlitePool};
pub async fn put_members(pool: &SqlitePool, membro: &Member) -> Result<(), sqlx::Error> {
    create_table_members(pool).await?;
    let member = membro;
    sqlx::query!(
        r#"
           UPDATE members
           SET nome = ?, idade= ?, email= ?, genero= ?, celular= ?,estado_civil= ?,endereco= ?
           WHERE id = ?
           "#,
        member.nome,
        member.idade,
        member.email,
        member.genero,
        member.celular,
        member.estado_civil,
        member.endereco,
        member.id
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn delete_members(pool: &SqlitePool, id: i64) -> Result<(), sqlx::Error> {
    create_table_members(pool).await?;
    sqlx::query!(
        r#"
        DELETE FROM members WHERE id = ?
           "#,
        id
    )
    .execute(pool)
    .await?;
    Ok(())
}
pub async fn del_members(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
) -> impl IntoResponse {
    if let Err(e) = create_table_members(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    match delete_members(&pool, path).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

pub async fn modify_members(
    State(pool): State<SqlitePool>,
    Path(path): Path<i64>,
    Json(mut membro): Json<Member>,
) -> impl IntoResponse {
    if let Err(e) = create_table_members(&pool).await {
        return format!("Erro no banco de dados {}", e).into_response();
    }
    membro.id = Some(path);
    match put_members(&pool, &membro).await {
        Ok(_) => axum::http::StatusCode::CREATED.into_response(),

        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Erro ao criar membro: {}", e),
        )
            .into_response(),
    }
}

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
pub async fn get_members(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<Member>>, (StatusCode, String)> {
    let members = sqlx::query_as!(Member, "SELECT * FROM members")
        .fetch_all(&pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Erro ao buscar membros {}", e),
            )
        })?;
    Ok(Json(members))
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
    id: Option<i64>,
    nome: String,
    idade: i64,
    email: String,
    genero: String,
    celular: String,
    estado_civil: String,
    endereco: String,
}

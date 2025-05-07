use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Json},
};
use base64::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use rand::RngCore;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

use sqlx::PgPool;
use std::sync::Arc;

use crate::{auth::verify_github_token, models::api_key::ApiKey, routes::RootState};

fn generate_api_key(mem_id: i32) -> String {
    let mut rng = rand::rng();
    let mut key = [0u8; 32];
    rng.fill_bytes(&mut key);

    let key = BASE64_STANDARD.encode(key);

    BASE64_STANDARD.encode(format!("{}_{}", mem_id, key))
}

pub fn hash_api_key(api_key: &str) -> String {
    hash(api_key, DEFAULT_COST).unwrap()
}

fn extract_bearer_token(headers: &HeaderMap) -> Option<String> {
    headers.get("authorization").and_then(|value| {
        value.to_str().ok().and_then(|s| {
            if s.to_lowercase().starts_with("bearer ") {
                Some(s[7..].to_string())
            } else {
                None
            }
        })
    })
}

async fn set_api_db(
    hashed_key: String,
    member_id: i32,
    pool: Arc<PgPool>,
) -> Result<(), sqlx::Error> {
    let query = sqlx::query_as::<_, ApiKey>(
        "
        INSERT INTO ApiKey (member_id, key_hash)
        VALUES ($1, $2)
        ON CONFLICT (member_id) DO UPDATE
            SET key_hash = $2
        RETURNING *",
    )
    .bind(member_id)
    .bind(hashed_key);

    query.fetch_one(pool.as_ref()).await?;

    Ok(())
}

#[derive(Serialize)]
struct ApiKeyResponse {
    api_key: String,
}

#[derive(Serialize, Deserialize)]
pub struct ApiKeyRequest {
    member_id: i32,
}

pub async fn get_api_key(
    headers: HeaderMap,
    State(state): State<RootState>,
    Json(body): Json<ApiKeyRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let pool = state.pool.clone();

    let token = match extract_bearer_token(&headers) {
        Some(token) => token,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    if let Ok(is_valid) = verify_github_token(token).await {
        if !is_valid {
            return Err(StatusCode::UNAUTHORIZED);
        }
    } else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    let api_key = generate_api_key(body.member_id);

    let payload = ApiKeyResponse {
        api_key: api_key.clone(),
    };

    let hashed_key = hash_api_key(&api_key);

    if (set_api_db(hashed_key, body.member_id, pool.clone()).await).is_err() {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(payload))
}

pub async fn verify_api_key(api_key: String, pool: Arc<PgPool>) -> Result<bool, sqlx::Error> {
    let decoded_api_key = BASE64_STANDARD
        .decode(api_key.clone())
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
        .unwrap_or_default();

    let member_id: Option<i32> = decoded_api_key
        .split_once('_')
        .and_then(|(member_id_str, _)| member_id_str.parse::<i32>().ok());

    let member_id = match member_id {
        Some(id) => id,
        None => {
            return Ok(false);
        }
    };

    let query = sqlx::query_as::<_, ApiKey>(
        "
        SELECT * FROM ApiKey WHERE member_id = $1",
    )
    .bind(member_id);

    let result = query.fetch_one(pool.as_ref()).await;

    let key_hash = match result {
        Ok(key) => key.key_hash,
        Err(_) => return Ok(false),
    };

    let is_valid = verify(&api_key, &key_hash).unwrap_or(false);

    Ok(is_valid)
}

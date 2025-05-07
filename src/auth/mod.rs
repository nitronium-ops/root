use axum::extract::{Query, State};
use axum::response::{IntoResponse, Redirect};
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use oauth2::basic::BasicClient;
use oauth2::reqwest;
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, RedirectUrl, Scope,
    TokenResponse, TokenUrl,
};
use reqwest::header::{AUTHORIZATION, USER_AGENT};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::env;
use std::error::Error;
use tracing::error;
use url::form_urlencoded;

use crate::routes::RootState;

struct GithubConfig {
    client_id: String,
    client_secret: String,
    redirect_url: String,
    frontend_redirect_url: String,
}

impl GithubConfig {
    fn from_env() -> Self {
        let _ = dotenv::dotenv();
        Self {
            client_id: env::var("GITHUB_CLIENT_ID").expect("GITHUB_CLIENT_ID must be set."),
            client_secret: env::var("GITHUB_CLIENT_SECRET")
                .expect("GITHUB_CLIENT_SECRET must be set."),
            redirect_url: env::var("GITHUB_REDIRECT_URL")
                .expect("GITHUB_REDIRECT_URL must be set."),
            frontend_redirect_url: env::var("FRONTEND_REDIRECT_URL")
                .expect("FRONTEND_REDIRECT_URL must be set."),
        }
    }
}

/// This function initiates the GitHub OAuth2 login process by redirecting the user to GitHub's authorization page.
/// It constructs the authorization URL with the necessary parameters and scopes, and then redirects the user to that URL.
pub async fn github_login() -> Redirect {
    let config = GithubConfig::from_env();

    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(ClientId::new(config.client_id))
        .set_client_secret(ClientSecret::new(config.client_secret))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(RedirectUrl::new(config.redirect_url).expect("Invalid redirect URL"));

    let (authorize_url, _csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        // This example is requesting access to the user's public repos and email.
        .add_scope(Scope::new("read:user".to_string()))
        .url();

    Redirect::to(authorize_url.as_str())
}

#[derive(Debug, Deserialize)]
pub struct AuthRequest {
    code: String,
}

#[derive(Debug, Serialize)]
pub struct TokenResponsePayload {
    access_token: String,
    member_id: MemberId,
    user: GithubUser,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GithubUser {
    login: String,
    id: u64,
    avatar_url: String,
    name: Option<String>,
    email: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct MemberId {
    member_id: i32,
}

/// This function handles the callback from GitHub after the user has authorized the application.
/// It exchanges the authorization code for an access token and then uses that token to fetch the user's information from GitHub.
/// If successful, it returns the access token and user information in a JSON response.
#[tracing::instrument(skip(state))]
pub async fn github_callback(
    Query(params): Query<AuthRequest>,
    State(state): State<RootState>,
) -> Result<impl IntoResponse, StatusCode> {
    let pool = state.pool.clone();

    let config = GithubConfig::from_env();

    let auth_url = AuthUrl::new("https://github.com/login/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");
    let token_url = TokenUrl::new("https://github.com/login/oauth/access_token".to_string())
        .expect("Invalid token endpoint URL");

    let client = BasicClient::new(ClientId::new(config.client_id))
        .set_client_secret(ClientSecret::new(config.client_secret))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(RedirectUrl::new(config.redirect_url).expect("Invalid redirect URL"));

    let http_client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("Client should build");

    let token_res = client
        .exchange_code(AuthorizationCode::new(params.code))
        .request_async(&http_client)
        .await
        .map_err(|err| {
            error!("Error exchanging code for token: {}", err);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let access_token = token_res.access_token().secret().to_string();

    let user_res = http_client
        .get("https://api.github.com/user")
        .header(USER_AGENT, "root")
        .header(AUTHORIZATION, format!("Bearer {}", access_token))
        .send()
        .await
        .map_err(|err| {
            error!("Error fetching user info: {}", err);
            StatusCode::UNAUTHORIZED
        })?;

    let user: GithubUser = user_res.json().await.map_err(|err| {
        error!("Failed to parse user info: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let query = sqlx::query_as::<_, MemberId>("SELECT member_id FROM Member WHERE github_id = $1")
        .bind(&user.login);

    let member_id = query.fetch_one(pool.as_ref()).await.map_err(|err| {
        error!("Error fetching member ID: {}", err);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let payload = TokenResponsePayload {
        access_token,
        member_id,
        user,
    };

    let response_data = serde_json::to_string(&payload).expect("Failed to serialize user data");

    let encoded_response_data = form_urlencoded::Serializer::new(String::new())
        .append_pair("user", BASE64_STANDARD.encode(&response_data).as_str())
        .finish();

    let redirect_url = format!("{}?{}", config.frontend_redirect_url, encoded_response_data);
    Ok(Redirect::to(&redirect_url))
}

/// This function verifies the GitHub token by making a request to the GitHub API.
/// It checks if the token is valid and returns a boolean indicating the result.
pub async fn verify_github_token(token: String) -> Result<bool, Box<dyn Error>> {
    let config = GithubConfig::from_env();
    let url = format!(
        "https://api.github.com/applications/{}/token",
        config.client_id
    );

    let client = Client::new();

    let response = client
        .post(&url)
        .header("User-Agent", "root")
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .basic_auth(config.client_id.clone(), Some(config.client_secret.clone()))
        .json(&serde_json::json!({
            "access_token": token
        }))
        .send()
        .await?;

    Ok(response.status() == StatusCode::OK)
}

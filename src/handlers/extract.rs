use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{self, request::Parts, StatusCode},
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub token: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthData
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(http::header::AUTHORIZATION)
            .and_then(|value| {
                let auth = value.to_str().unwrap_or("");
                if auth.contains("Bearer") {
                    auth.get(6..)
                } else {
                    None
                }
            });

        match auth_header {
            Some(token) => Ok(AuthData {
                token: token.trim().to_string(),
            }),
            None => return Err((StatusCode::UNAUTHORIZED, "Unauthorized")),
        }
    }
}

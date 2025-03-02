use gloo_net::http::{Request, Response};
use crate::models::auth::{AuthRequest, AuthResponse};
use gloo_net::Error;

pub async fn login(request: AuthRequest) -> Result<AuthResponse, Error> {
    let response: Response = Request::post("/api/login")
        .json(&request)
        .unwrap()
        .send()
        .await?;

    response.json().await
}

pub async fn signup(request: AuthRequest) -> Result<AuthResponse, Error> {
    let response: Response = Request::post("/api/signup")
        .json(&request)
        .unwrap()
        .send()
        .await?;

    response.json().await
}
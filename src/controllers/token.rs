use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::api_response::ApiResponse;
use crate::models::token::{CreateTokenRequest, CreateTokenResponse, MintTokenRequest, MintTokenResponse};
use crate::services::token as token_service;


pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    if payload.mintAuthority.is_empty() || payload.mint.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<CreateTokenResponse>::error("Missing required fields")));
    }
    // Validate mintAuthority and mint (base58, 32 bytes)
    if bs58::decode(&payload.mintAuthority).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<CreateTokenResponse>::error("Invalid base58 mint authority pubkey")));
    }
    if bs58::decode(&payload.mint).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<CreateTokenResponse>::error("Invalid base58 mint pubkey")));
    }
    match token_service::create_token_instruction(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<CreateTokenResponse>::error(&e))),
    }
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    if payload.mint.is_empty() || payload.destination.is_empty() || payload.authority.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<MintTokenResponse>::error("Missing required fields")));
    }
    // Validate mint, destination, authority (base58, 32 bytes)
    if bs58::decode(&payload.mint).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<MintTokenResponse>::error("Invalid base58 mint pubkey")));
    }
    if bs58::decode(&payload.destination).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<MintTokenResponse>::error("Invalid base58 destination pubkey")));
    }
    if bs58::decode(&payload.authority).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<MintTokenResponse>::error("Invalid base58 authority pubkey")));
    }
    match token_service::mint_token_instruction(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<MintTokenResponse>::error(&e))),
    }
} 

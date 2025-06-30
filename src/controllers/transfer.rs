use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::api_response::ApiResponse;
use crate::models::transfer::{SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse};
use crate::services::transfer as transfer_service;


pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    if payload.from.is_empty() || payload.to.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendSolResponse>::error("Missing required fields")));
    }
    // Validate from and to (base58, 32 bytes)
    if bs58::decode(&payload.from).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendSolResponse>::error("Invalid base58 from pubkey")));
    }
    if bs58::decode(&payload.to).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendSolResponse>::error("Invalid base58 to pubkey")));
    }
    match transfer_service::send_sol_instruction(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendSolResponse>::error(&e))),
    }
}

pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> impl IntoResponse {
    if payload.destination.is_empty() || payload.mint.is_empty() || payload.owner.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendTokenResponse>::error("Missing required fields")));
    }
    // Validate destination, mint, owner (base58, 32 bytes)
    if bs58::decode(&payload.destination).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendTokenResponse>::error("Invalid base58 destination pubkey")));
    }
    if bs58::decode(&payload.mint).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendTokenResponse>::error("Invalid base58 mint pubkey")));
    }
    if bs58::decode(&payload.owner).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendTokenResponse>::error("Invalid base58 owner pubkey")));
    }
    match transfer_service::send_token_instruction(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<SendTokenResponse>::error(&e))),
    }
} 
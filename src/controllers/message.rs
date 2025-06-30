use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::api_response::ApiResponse;
use crate::models::message::{SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse};
use crate::services::message as message_service;
use base64::engine::general_purpose;
use base64::Engine;

pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SignMessageResponse>::error("Missing required fields")));
    }
    // Validate secret key format (base58, 64 bytes)
    if bs58::decode(&payload.secret).into_vec().map(|v| v.len() == 64).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<SignMessageResponse>::error("Invalid base58 secret key")));
    }
    match message_service::sign_message(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<SignMessageResponse>::error(&e))),
    }
}

pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<VerifyMessageResponse>::error("Missing required fields")));
    }
    // Validate pubkey (base58, 32 bytes)
    if bs58::decode(&payload.pubkey).into_vec().map(|v| v.len() == 32).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<VerifyMessageResponse>::error("Invalid base58 pubkey")));
    }
    // Validate signature (base64, 64 bytes)
    if general_purpose::STANDARD.decode(&payload.signature).map(|v| v.len() == 64).unwrap_or(false) == false {
        return (StatusCode::BAD_REQUEST, Json(ApiResponse::<VerifyMessageResponse>::error("Invalid base64 signature")));
    }
    match message_service::verify_message(&payload) {
        Ok(resp) => (StatusCode::OK, Json(ApiResponse::success(resp))),
        Err(e) => (StatusCode::BAD_REQUEST, Json(ApiResponse::<VerifyMessageResponse>::error(&e))),
    }
} 

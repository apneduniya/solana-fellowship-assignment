use axum::{Json, response::IntoResponse};
use crate::models::api_response::ApiResponse;
use crate::models::message::{SignMessageRequest, SignMessageResponse, VerifyMessageRequest, VerifyMessageResponse};
use crate::services::message as message_service;


pub async fn sign_message(Json(payload): Json<SignMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.secret.is_empty() {
        return Json(ApiResponse::<SignMessageResponse>::error("Missing required fields"));
    }
    match message_service::sign_message(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<SignMessageResponse>::error(&e)),
    }
}


pub async fn verify_message(Json(payload): Json<VerifyMessageRequest>) -> impl IntoResponse {
    if payload.message.is_empty() || payload.signature.is_empty() || payload.pubkey.is_empty() {
        return Json(ApiResponse::<VerifyMessageResponse>::error("Missing required fields"));
    }
    match message_service::verify_message(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<VerifyMessageResponse>::error(&e)),
    }
} 

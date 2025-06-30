use axum::{Json, response::IntoResponse};
use crate::models::api_response::ApiResponse;
use crate::models::transfer::{SendSolRequest, SendSolResponse, SendTokenRequest, SendTokenResponse};
use crate::services::transfer as transfer_service;


pub async fn send_sol(Json(payload): Json<SendSolRequest>) -> impl IntoResponse {
    match transfer_service::send_sol_instruction(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<SendSolResponse>::error(&e)),
    }
}


pub async fn send_token(Json(payload): Json<SendTokenRequest>) -> impl IntoResponse {
    match transfer_service::send_token_instruction(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<SendTokenResponse>::error(&e)),
    }
} 
use axum::{Json, response::IntoResponse};
use crate::models::api_response::ApiResponse;
use crate::services::keypair as keypair_service;


pub async fn generate_keypair() -> impl IntoResponse {
    let keypair = keypair_service::generate_keypair();
    Json(ApiResponse::success(keypair))
} 

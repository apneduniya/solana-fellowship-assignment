use axum::{Json, response::IntoResponse, http::StatusCode};
use crate::models::api_response::ApiResponse;
use crate::services::keypair as keypair_service;


pub async fn generate_keypair() -> impl IntoResponse {
    // Keypair generation should not fail, but handle as 500 if it does
    let keypair = keypair_service::generate_keypair();
    (StatusCode::OK, Json(ApiResponse::success(keypair)))
} 

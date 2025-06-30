use axum::{Json, response::IntoResponse};
use crate::models::api_response::ApiResponse;
use crate::models::token::{CreateTokenRequest, CreateTokenResponse, MintTokenRequest, MintTokenResponse};
use crate::services::token as token_service;


pub async fn create_token(Json(payload): Json<CreateTokenRequest>) -> impl IntoResponse {
    match token_service::create_token_instruction(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<CreateTokenResponse>::error(&e)),
    }
}

pub async fn mint_token(Json(payload): Json<MintTokenRequest>) -> impl IntoResponse {
    match token_service::mint_token_instruction(&payload) {
        Ok(resp) => Json(ApiResponse::success(resp)),
        Err(e) => Json(ApiResponse::<MintTokenResponse>::error(&e)),
    }
} 

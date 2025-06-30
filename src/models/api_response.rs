use serde::Serialize;

#[derive(Serialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Success { success: bool, data: T },
    Error { success: bool, error: String },
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        ApiResponse::Success { success: true, data }
    }
    pub fn error(msg: &str) -> Self {
        ApiResponse::Error { success: false, error: msg.to_string() }
    }
} 
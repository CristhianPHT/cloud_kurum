use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ImageUploadRequest {
    pub filename: String,
    pub content_type: String,
}
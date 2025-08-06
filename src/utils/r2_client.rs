use crate::models::image::{HttpMethod, ImageError, R2Config, R2PresignedUrlRequest};
use aws_sdk_s3::operation::put_object::PutObjectOutput;
use aws_sdk_s3::presigning::PresigningConfig;
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::{Client, Config, Credentials, Region};
use chrono::{DateTime, Utc};
use std::time::Duration;

#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket_name: String,
}

impl R2Client {
    pub fn new(config: R2Config) -> Result<Self, ImageError> {
        let credentials = Credentials::new(
            config.access_key_id,
            config.secret_access_key,
            None,
            None,
            "r2-credentials",
        );

        let s3_config = Config::builder()
            .credentials_provider(credentials)
            .endpoint_url(&config.endpoint)
            .region(Region::new("auto"))
            .build();

        let client = Client::from_conf(s3_config);

        Ok(R2Client {
            client,
            bucket_name: config.bucket_name,
        })
    }

    /// Generate a pre-signed URL for uploading an image
    pub async fn generate_upload_url(
        &self,
        key: &str,
        expires_in: Duration,
        content_type: &str,
    ) -> Result<String, ImageError> {
        let presigning_config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| ImageError::R2Error(format!("Presigning config error: {}", e)))?;

        let request = self
            .client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .content_type(content_type)
            .presigned(presigning_config)
            .await
            .map_err(|e| ImageError::R2Error(format!("Failed to generate upload URL: {}", e)))?;

        Ok(request.uri().to_string())
    }

    /// Generate a pre-signed URL for accessing/downloading an image
    pub async fn generate_access_url(
        &self,
        key: &str,
        expires_in: Duration,
    ) -> Result<String, ImageError> {
        let presigning_config = PresigningConfig::expires_in(expires_in)
            .map_err(|e| ImageError::R2Error(format!("Presigning config error: {}", e)))?;

        let request = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| ImageError::R2Error(format!("Failed to generate access URL: {}", e)))?;

        Ok(request.uri().to_string())
    }

    /// Upload an image directly (for server-side uploads)
    pub async fn upload_image(
        &self,
        key: &str,
        data: Vec<u8>,
        content_type: &str,
    ) -> Result<PutObjectOutput, ImageError> {
        let body = ByteStream::from(data);

        self.client
            .put_object()
            .bucket(&self.bucket_name)
            .key(key)
            .body(body)
            .content_type(content_type)
            .send()
            .await
            .map_err(|e| ImageError::R2Error(format!("Failed to upload image: {}", e)))
    }

    /// Delete an image from R2
    pub async fn delete_image(&self, key: &str) -> Result<(), ImageError> {
        self.client
            .delete_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
            .map_err(|e| ImageError::R2Error(format!("Failed to delete image: {}", e)))?;

        Ok(())
    }

    /// Check if an image exists in R2
    pub async fn image_exists(&self, key: &str) -> Result<bool, ImageError> {
        match self
            .client
            .head_object()
            .bucket(&self.bucket_name)
            .key(key)
            .send()
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                let error_str = e.to_string();
                if error_str.contains("NotFound") || error_str.contains("404") {
                    Ok(false)
                } else {
                    Err(ImageError::R2Error(format!(
                        "Failed to check image existence: {}",
                        e
                    )))
                }
            }
        }
    }

    /// Generate a unique key for storing images
    pub fn generate_image_key(user_id: i32, category: &str, file_name: &str) -> String {
        let timestamp = Utc::now().timestamp();
        let extension = file_name.split('.').last().unwrap_or("jpg");

        format!(
            "images/{}/{}/{}-{}.{}",
            category,
            user_id,
            timestamp,
            uuid::Uuid::new_v4(),
            extension
        )
    }

    /// Validate image file type
    pub fn is_valid_image_type(content_type: &str) -> bool {
        matches!(
            content_type,
            "image/jpeg" | "image/jpg" | "image/png" | "image/webp" | "image/gif"
        )
    }

    /// Validate image file size (max 10MB by default)
    pub fn is_valid_file_size(size: u64, max_size: Option<u64>) -> bool {
        let max = max_size.unwrap_or(10 * 1024 * 1024); // 10MB default
        size <= max
    }

    /// Get public URL for public images (without pre-signing)
    pub fn get_public_url(&self, key: &str) -> String {
        format!("https://{}.r2.dev/{}", self.bucket_name, key)
    }
}

/// Helper function to extract file extension from filename
pub fn get_file_extension(filename: &str) -> Option<&str> {
    filename.split('.').last()
}

/// Helper function to generate content type from file extension
pub fn content_type_from_extension(extension: &str) -> &str {
    match extension.to_lowercase().as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "svg" => "image/svg+xml",
        _ => "application/octet-stream",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_image_key() {
        let key = R2Client::generate_image_key(123, "profile", "avatar.jpg");
        assert!(key.starts_with("images/profile/123/"));
        assert!(key.ends_with(".jpg"));
    }

    #[test]
    fn test_is_valid_image_type() {
        assert!(R2Client::is_valid_image_type("image/jpeg"));
        assert!(R2Client::is_valid_image_type("image/png"));
        assert!(!R2Client::is_valid_image_type("text/plain"));
    }

    #[test]
    fn test_is_valid_file_size() {
        assert!(R2Client::is_valid_file_size(1024, None)); // 1KB
        assert!(R2Client::is_valid_file_size(5 * 1024 * 1024, None)); // 5MB
        assert!(!R2Client::is_valid_file_size(15 * 1024 * 1024, None)); // 15MB
    }

    #[test]
    fn test_content_type_from_extension() {
        assert_eq!(content_type_from_extension("jpg"), "image/jpeg");
        assert_eq!(content_type_from_extension("png"), "image/png");
        assert_eq!(content_type_from_extension("gif"), "image/gif");
    }
}

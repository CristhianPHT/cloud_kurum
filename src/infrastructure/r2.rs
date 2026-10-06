
use aws_sdk_s3::{ config::Region, config::Credentials, Client };
use aws_sdk_s3::presigning::PresigningConfig;
use std::time::Duration;

pub fn create_r2_client() -> Client {   // Decisión cliente r2...
  let access_key = std::env::var("R2_ACCESS_KEY_ID")
    .expect("R2_ACCESS_KEY_ID no está configurado");

  let secret_key = std::env::var("R2_SECRET_ACCESS_KEY")
    .expect("R2_SECRET_ACCESS_KEY no está configurado");

  let endpoint = std::env::var("R2_ENDPOINT")
    .expect("R2_ENDPOINT no está configurado");

  let credentials = Credentials::new(
    access_key,
    secret_key,
    None,
    None,
    "r2",
  );

  let config = aws_sdk_s3::Config::builder()
    .behavior_version_latest()
    .region(Region::new("auto"))
    .endpoint_url(endpoint)
    .credentials_provider(credentials)
    .build();

  Client::from_conf(config)
}

pub async fn create_presigned_put_url( r2: &Client, bucket: &str, key: &str, content_type: &str
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> { // el error requiere un emun para mejor uso
  let presigned_request = r2
      .put_object()
      .bucket(bucket)
      .key(key)
      .content_type(content_type)
      .presigned(
          PresigningConfig::expires_in(Duration::from_secs(300))?
      )
      .await?;
  Ok(presigned_request.uri().to_string())
}

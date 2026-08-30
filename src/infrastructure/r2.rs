
use aws_sdk_s3::{ config::Region, config::Credentials, Client };

pub fn create_r2_client() -> Client {
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
    // .behavior_version_latest()
    .region(Region::new("auto"))
    .endpoint_url(endpoint)
    .credentials_provider(credentials)
    .build();

  Client::from_conf(config)
}
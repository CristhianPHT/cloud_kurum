use aws_sdk_s3::Client;
use uuid::Uuid;

use crate::infrastructure::r2::create_presigned_put_url;

pub struct ImageR2Service;

impl ImageR2Service {
  pub async fn create_upload( r2: &Client, filename: &str, content_type: &str,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {

    let extension = filename
      .rsplit('.')
      .next()
      .filter(|extension| !extension.is_empty())
      .unwrap_or("bin");

    let key = format!(
      "public/images/{}.{}",
      Uuid::new_v4(),
      extension
    );

    let bucket = std::env::var("R2_BUCKET")
      .expect("R2_BUCKET no está configurado");

    let upload_url = create_presigned_put_url(
      r2,
      &bucket,
      &key,
      content_type,
    )
    .await?;

    Ok((key, upload_url))
  }
}
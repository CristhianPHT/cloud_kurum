use actix_web::{get, post, web, HttpResponse, Responder};
use std::time::Duration;

#[get("/image/test/upload-url")]
pub async fn test_upload_url( r2: web::Data<aws_sdk_s3::Client>) -> impl Responder {
  let bucket = std::env::var("R2_BUCKET")
    .expect("R2_BUCKET no está configurado");

  let key = "public/image/imagen-prueba.jpg";
  // let key = format!("public/{}", path.into_inner());

  let presigned_request = r2
    .put_object()
    .bucket(bucket)
    .key(key)
    .presigned(
      aws_sdk_s3::presigning::PresigningConfig::expires_in(
        Duration::from_secs(300)
      )
      .expect("Duración de presigned URL inválida")
    )
    .await;
  match presigned_request {
    Ok(request) => HttpResponse::Ok().json(request.uri()),
    Err(error) => {
      eprintln!("Error generando presigned URL: {error:?}");
      HttpResponse::InternalServerError().json("No se pudo generar la URL")
    }
  }
}

#[get("/image/test/r2")]
pub async fn test_r2( r2: web::Data<aws_sdk_s3::Client> ) -> impl Responder {
  let bucket = std::env::var("R2_BUCKET")
    .expect("R2_BUCKET no está configurado");
  match r2
    .list_objects_v2()
    .bucket(bucket)
    .send()
    .await
  {
  Ok(response) => {
    println!("R2 respondió correctamente");
    println!("Objetos: {:?}", response.contents());
    HttpResponse::Ok().json("R2 conectado correctamente")
  }
  Err(error) => {
    eprintln!("Error comunicando con R2: {error:?}");
    HttpResponse::InternalServerError()
      .json("Error comunicando con R2")
    }
  }
}

use crate::web::dto::image_r2::ImageUploadRequest;
use crate::services::image_r2::ImageR2Service;

#[post("/image/upload")]
pub async fn create_upload(
    data: web::Json<ImageUploadRequest>,
    r2: web::Data<aws_sdk_s3::Client>,
) -> impl Responder {

    match ImageR2Service::create_upload(
        &r2,
        &data.filename,
        &data.content_type,
    )
    .await
    {
        Ok((key, upload_url)) => {
            HttpResponse::Ok().json(serde_json::json!({
                "key": key,
                "upload_url": upload_url,
            }))
        }

        Err(error) => {
            eprintln!("Error creando upload: {error}");

            HttpResponse::InternalServerError()
                .json("No se pudo generar la URL de subida")
        }
    }
}

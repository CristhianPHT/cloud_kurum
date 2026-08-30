use actix_web::{get, web, HttpResponse, Responder};

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
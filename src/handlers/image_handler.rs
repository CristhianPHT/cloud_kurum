use crate::models::image::{ImageAccessRequest, ImageCategory, ImageError, ImageUploadRequest};
use crate::services::image_service::ImageService;
use crate::utils::r2_client::R2Client;
use actix_multipart::Multipart;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse, Responder};
use diesel::PgConnection;
use futures_util::TryStreamExt;
use serde_json::json;
use std::io::Write;

// GET /api/images/upload-url - Generate upload URL for client-side upload
#[post("/upload-url")]
pub async fn generate_upload_url(
    req: HttpRequest,
    request: web::Json<ImageUploadRequest>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    // Extract user ID from JWT token (implement your auth middleware)
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    match image_service
        .generate_upload_url(&mut conn, user_id, request.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(ImageError::InvalidFormat) => {
            HttpResponse::BadRequest().json(json!({"error": "Invalid image format"}))
        }
        Err(ImageError::TooLarge) => {
            HttpResponse::BadRequest().json(json!({"error": "Image too large"}))
        }
        Err(ImageError::Unauthorized) => {
            HttpResponse::Forbidden().json(json!({"error": "Unauthorized"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// POST /api/images/access-url - Generate access URL for viewing images
#[post("/access-url")]
pub async fn generate_access_url(
    req: HttpRequest,
    request: web::Json<ImageAccessRequest>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    match image_service
        .generate_access_url(&mut conn, user_id, request.into_inner())
        .await
    {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(ImageError::NotFound) => {
            HttpResponse::NotFound().json(json!({"error": "Image not found"}))
        }
        Err(ImageError::Unauthorized) => {
            HttpResponse::Forbidden().json(json!({"error": "Unauthorized access"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// POST /api/images/upload - Direct server-side upload
#[post("/upload")]
pub async fn upload_image_direct(
    req: HttpRequest,
    mut payload: Multipart,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    let mut file_data = Vec::new();
    let mut file_name = String::new();
    let mut content_type = String::new();
    let mut category = ImageCategory::ProfilePicture; // Default

    // Process multipart form data
    while let Some(mut field) = payload.try_next().await.unwrap() {
        let field_name = field.name();

        match field_name {
            "file" => {
                file_name = field
                    .content_disposition()
                    .get_filename()
                    .unwrap_or("image.jpg")
                    .to_string();

                content_type = field
                    .content_type()
                    .map(|ct| ct.to_string())
                    .unwrap_or_else(|| "image/jpeg".to_string());

                // Read file data
                while let Some(chunk) = field.try_next().await.unwrap() {
                    file_data.extend_from_slice(&chunk);
                }
            }
            "category" => {
                let mut category_bytes = Vec::new();
                while let Some(chunk) = field.try_next().await.unwrap() {
                    category_bytes.extend_from_slice(&chunk);
                }
                let category_str = String::from_utf8(category_bytes).unwrap_or_default();
                category = match category_str.as_str() {
                    "profile" => ImageCategory::ProfilePicture,
                    "book_cover" => ImageCategory::BookCover,
                    "chapter" => ImageCategory::ChapterImage,
                    "scan_logo" => ImageCategory::ScanLogo,
                    _ => ImageCategory::ProfilePicture,
                };
            }
            _ => {}
        }
    }

    if file_data.is_empty() {
        return HttpResponse::BadRequest().json(json!({"error": "No file provided"}));
    }

    match image_service
        .upload_image_direct(
            &mut conn,
            user_id,
            file_name,
            file_data,
            content_type,
            category,
        )
        .await
    {
        Ok(image_id) => HttpResponse::Ok().json(json!({"image_id": image_id})),
        Err(ImageError::InvalidFormat) => {
            HttpResponse::BadRequest().json(json!({"error": "Invalid image format"}))
        }
        Err(ImageError::TooLarge) => {
            HttpResponse::BadRequest().json(json!({"error": "Image too large"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// GET /api/images/my-images - Get user's images
#[get("/my-images")]
pub async fn get_user_images(
    req: HttpRequest,
    query: web::Query<UserImagesQuery>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    let category = query.category.as_ref().and_then(|c| match c.as_str() {
        "profile" => Some(ImageCategory::ProfilePicture),
        "book_cover" => Some(ImageCategory::BookCover),
        "chapter" => Some(ImageCategory::ChapterImage),
        "scan_logo" => Some(ImageCategory::ScanLogo),
        _ => None,
    });

    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    match image_service.get_user_images(&mut conn, user_id, category, page, per_page) {
        Ok(images) => HttpResponse::Ok().json(json!({"images": images})),
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// PUT /api/images/{image_id} - Update image metadata
#[put("/{image_id}")]
pub async fn update_image_metadata(
    req: HttpRequest,
    path: web::Path<String>,
    update_data: web::Json<UpdateImageRequest>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    let image_id = path.into_inner();

    match image_service.update_image_metadata(
        &mut conn,
        user_id,
        &image_id,
        update_data.new_name.clone(),
        update_data.is_public,
    ) {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Image updated successfully"})),
        Err(ImageError::NotFound) => {
            HttpResponse::NotFound().json(json!({"error": "Image not found"}))
        }
        Err(ImageError::Unauthorized) => {
            HttpResponse::Forbidden().json(json!({"error": "Unauthorized"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// DELETE /api/images/{image_id} - Delete image
#[delete("/{image_id}")]
pub async fn delete_image(
    req: HttpRequest,
    path: web::Path<String>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    let image_id = path.into_inner();

    match image_service
        .delete_image(&mut conn, user_id, &image_id)
        .await
    {
        Ok(_) => HttpResponse::Ok().json(json!({"message": "Image deleted successfully"})),
        Err(ImageError::NotFound) => {
            HttpResponse::NotFound().json(json!({"error": "Image not found"}))
        }
        Err(ImageError::Unauthorized) => {
            HttpResponse::Forbidden().json(json!({"error": "Unauthorized"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// GET /api/images/{image_id}/thumbnail - Generate thumbnail
#[get("/{image_id}/thumbnail")]
pub async fn generate_thumbnail(
    req: HttpRequest,
    path: web::Path<String>,
    query: web::Query<ThumbnailQuery>,
    image_service: web::Data<ImageService>,
    mut conn: web::Data<PgConnection>,
) -> impl Responder {
    let user_id = match extract_user_id_from_request(&req) {
        Ok(id) => id,
        Err(_) => return HttpResponse::Unauthorized().json(json!({"error": "Unauthorized"})),
    };

    let image_id = path.into_inner();
    let size = query.size.unwrap_or(150);

    match image_service
        .generate_thumbnail(&mut conn, &image_id, size)
        .await
    {
        Ok(thumbnail_key) => HttpResponse::Ok().json(json!({"thumbnail_key": thumbnail_key})),
        Err(ImageError::NotFound) => {
            HttpResponse::NotFound().json(json!({"error": "Image not found"}))
        }
        Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
    }
}

// Helper structs for query parameters
#[derive(serde::Deserialize)]
pub struct UserImagesQuery {
    pub category: Option<String>,
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(serde::Deserialize)]
pub struct UpdateImageRequest {
    pub new_name: Option<String>,
    pub is_public: Option<bool>,
}

#[derive(serde::Deserialize)]
pub struct ThumbnailQuery {
    pub size: Option<u32>,
}

// Helper function to extract user ID from JWT token
// You should implement this based on your auth middleware
fn extract_user_id_from_request(req: &HttpRequest) -> Result<i32, &'static str> {
    // This is a placeholder - implement your JWT token extraction logic
    // You might want to use a middleware for this instead

    if let Some(auth_header) = req.headers().get("Authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                // TODO: Validate JWT token and extract user_id
                // For now, return a placeholder
                return Ok(1); // This should be the actual user ID from the token
            }
        }
    }

    Err("No valid authorization header")
}

// Configuration function to register all image routes
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/images")
            .service(generate_upload_url)
            .service(generate_access_url)
            .service(upload_image_direct)
            .service(get_user_images)
            .service(update_image_metadata)
            .service(delete_image)
            .service(generate_thumbnail),
    );
}

// Health check endpoint for image service
#[get("/health")]
pub async fn image_service_health() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "status": "healthy",
        "service": "image_service",
        "timestamp": chrono::Utc::now()
    }))
}

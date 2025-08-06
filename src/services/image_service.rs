use crate::models::image::{
    ImageAccessRequest, ImageAccessResponse, ImageCategory, ImageError, ImageMetadata,
    ImageUploadRequest, ImageUploadResponse, NewImageMetadata, R2Config,
};
use crate::utils::r2_client::R2Client;
use chrono::{Duration, NaiveDateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;
use std::time::Duration as StdDuration;
use uuid::Uuid;

pub struct ImageService {
    r2_client: R2Client,
}

impl ImageService {
    pub fn new(r2_config: R2Config) -> Result<Self, ImageError> {
        let r2_client = R2Client::new(r2_config)?;
        Ok(ImageService { r2_client })
    }

    /// Generate upload URL for client-side upload
    pub async fn generate_upload_url(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        request: ImageUploadRequest,
    ) -> Result<ImageUploadResponse, ImageError> {
        // Validate request
        self.validate_upload_request(&request)?;

        // Check user permissions
        self.check_user_permissions(conn, user_id, &request.category)?;

        // Generate unique image ID and R2 key
        let image_id = Uuid::new_v4().to_string();
        let r2_key = self.generate_r2_key(user_id, &request.category, &request.file_name);

        // Generate pre-signed upload URL (expires in 1 hour)
        let expires_in = StdDuration::from_secs(3600);
        let upload_url = self
            .r2_client
            .generate_upload_url(&r2_key, expires_in, &request.content_type)
            .await?;

        // Save metadata to database
        let metadata = NewImageMetadata {
            image_id: image_id.clone(),
            owner_id: user_id,
            file_name: request.file_name,
            file_size: request.file_size,
            content_type: request.content_type,
            category: format!("{:?}", request.category),
            r2_key: r2_key.clone(),
            is_public: self.is_category_public(&request.category),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.save_image_metadata(conn, metadata)?;

        Ok(ImageUploadResponse {
            upload_url,
            image_id,
            expires_at: (Utc::now() + Duration::hours(1)).naive_utc(),
        })
    }

    /// Generate access URL for viewing images
    pub async fn generate_access_url(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        request: ImageAccessRequest,
    ) -> Result<ImageAccessResponse, ImageError> {
        // Get image metadata from database
        let metadata = self.get_image_metadata(conn, &request.image_id)?;

        // Check access permissions
        self.check_access_permissions(user_id, &metadata)?;

        // Generate pre-signed access URL (expires in 4 hours)
        let expires_in = StdDuration::from_secs(14400);
        let access_url = self
            .r2_client
            .generate_access_url(&metadata.r2_key, expires_in)
            .await?;

        Ok(ImageAccessResponse {
            access_url,
            expires_at: (Utc::now() + Duration::hours(4)).naive_utc(),
        })
    }

    /// Upload image directly (server-side upload)
    pub async fn upload_image_direct(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        file_name: String,
        file_data: Vec<u8>,
        content_type: String,
        category: ImageCategory,
    ) -> Result<String, ImageError> {
        // Validate
        if !R2Client::is_valid_image_type(&content_type) {
            return Err(ImageError::InvalidFormat);
        }

        if !R2Client::is_valid_file_size(file_data.len() as u64, None) {
            return Err(ImageError::TooLarge);
        }

        // Generate R2 key and upload
        let image_id = Uuid::new_v4().to_string();
        let r2_key = self.generate_r2_key(user_id, &category, &file_name);

        self.r2_client
            .upload_image(&r2_key, file_data.clone(), &content_type)
            .await?;

        // Save metadata
        let metadata = NewImageMetadata {
            image_id: image_id.clone(),
            owner_id: user_id,
            file_name,
            file_size: file_data.len() as u64,
            content_type,
            category: format!("{:?}", category),
            r2_key,
            is_public: self.is_category_public(&category),
            created_at: Utc::now().naive_utc(),
            updated_at: Utc::now().naive_utc(),
        };

        self.save_image_metadata(conn, metadata)?;

        Ok(image_id)
    }

    /// Delete image
    pub async fn delete_image(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        image_id: &str,
    ) -> Result<(), ImageError> {
        let metadata = self.get_image_metadata(conn, image_id)?;

        // Check permissions (only owner can delete)
        if metadata.owner_id != user_id {
            return Err(ImageError::Unauthorized);
        }

        // Delete from R2
        self.r2_client.delete_image(&metadata.r2_key).await?;

        // Delete metadata from database
        self.delete_image_metadata(conn, image_id)?;

        Ok(())
    }

    /// Get user's images
    pub fn get_user_images(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        category: Option<ImageCategory>,
        page: i64,
        per_page: i64,
    ) -> Result<Vec<ImageMetadata>, ImageError> {
        use crate::schema::images::dsl::*;

        let mut query = images.filter(owner_id.eq(user_id)).into_boxed();

        if let Some(cat) = category {
            query = query.filter(category.eq(format!("{:?}", cat)));
        }

        let offset = (page - 1) * per_page;
        query
            .offset(offset)
            .limit(per_page)
            .order(created_at.desc())
            .load::<ImageMetadata>(conn)
            .map_err(|e| ImageError::DatabaseError(e.to_string()))
    }

    /// Update image metadata
    pub fn update_image_metadata(
        &self,
        conn: &mut PgConnection,
        user_id: i32,
        image_id: &str,
        new_name: Option<String>,
        is_public: Option<bool>,
    ) -> Result<(), ImageError> {
        use crate::schema::images::dsl::*;

        let metadata = self.get_image_metadata(conn, image_id)?;

        if metadata.owner_id != user_id {
            return Err(ImageError::Unauthorized);
        }

        let mut changeset = diesel::update(images.filter(image_id.eq(image_id)));

        if let Some(name) = new_name {
            changeset = changeset.set(file_name.eq(name));
        }

        if let Some(public) = is_public {
            changeset = changeset.set(is_public.eq(public));
        }

        changeset
            .set(updated_at.eq(Utc::now().naive_utc()))
            .execute(conn)
            .map_err(|e| ImageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    // Private helper methods

    fn validate_upload_request(&self, request: &ImageUploadRequest) -> Result<(), ImageError> {
        if !R2Client::is_valid_image_type(&request.content_type) {
            return Err(ImageError::InvalidFormat);
        }

        if !R2Client::is_valid_file_size(request.file_size, None) {
            return Err(ImageError::TooLarge);
        }

        if request.file_name.is_empty() {
            return Err(ImageError::InvalidFormat);
        }

        Ok(())
    }

    fn check_user_permissions(
        &self,
        _conn: &mut PgConnection,
        _user_id: i32,
        category: &ImageCategory,
    ) -> Result<(), ImageError> {
        // Implement your business logic here
        // For example, check if user has permission to upload certain categories
        match category {
            ImageCategory::ProfilePicture => Ok(()), // Everyone can upload profile pictures
            ImageCategory::BookCover => Ok(()),      // Check if user can create books
            ImageCategory::ChapterImage => Ok(()),   // Check if user owns the book
            ImageCategory::ScanLogo => Ok(()),       // Check if user is scan member
        }
    }

    fn check_access_permissions(
        &self,
        user_id: i32,
        metadata: &ImageMetadata,
    ) -> Result<(), ImageError> {
        // Public images are accessible to everyone
        if metadata.is_public {
            return Ok(());
        }

        // Private images only accessible to owner
        if metadata.owner_id == user_id {
            return Ok(());
        }

        // Add more complex permission logic here if needed
        // For example, friends can see profile pictures, etc.

        Err(ImageError::Unauthorized)
    }

    fn generate_r2_key(&self, user_id: i32, category: &ImageCategory, file_name: &str) -> String {
        let category_str = match category {
            ImageCategory::ProfilePicture => "profiles",
            ImageCategory::BookCover => "books",
            ImageCategory::ChapterImage => "chapters",
            ImageCategory::ScanLogo => "scans",
        };

        R2Client::generate_image_key(user_id, category_str, file_name)
    }

    fn is_category_public(&self, category: &ImageCategory) -> bool {
        match category {
            ImageCategory::ProfilePicture => false, // Private by default
            ImageCategory::BookCover => true,       // Public book covers
            ImageCategory::ChapterImage => false,   // Private chapter images
            ImageCategory::ScanLogo => true,        // Public scan logos
        }
    }

    fn save_image_metadata(
        &self,
        conn: &mut PgConnection,
        metadata: NewImageMetadata,
    ) -> Result<(), ImageError> {
        use crate::schema::images::dsl::*;
        use diesel::insert_into;

        insert_into(images)
            .values(&metadata)
            .execute(conn)
            .map_err(|e| ImageError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    fn get_image_metadata(
        &self,
        conn: &mut PgConnection,
        image_id_val: &str,
    ) -> Result<ImageMetadata, ImageError> {
        use crate::schema::images::dsl::*;

        images
            .filter(image_id.eq(image_id_val))
            .first::<ImageMetadata>(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ImageError::NotFound,
                _ => ImageError::DatabaseError(e.to_string()),
            })
    }

    fn delete_image_metadata(
        &self,
        conn: &mut PgConnection,
        image_id_val: &str,
    ) -> Result<(), ImageError> {
        use crate::schema::images::dsl::*;
        use diesel::delete;

        delete(images.filter(image_id.eq(image_id_val)))
            .execute(conn)
            .map_err(|e| ImageError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

// Helper functions for image processing
impl ImageService {
    /// Resize image if needed (you might want to add image processing)
    pub async fn process_image(
        &self,
        image_data: Vec<u8>,
        max_width: Option<u32>,
        max_height: Option<u32>,
    ) -> Result<Vec<u8>, ImageError> {
        // Placeholder for image processing logic
        // You could use libraries like `image` crate here
        // For now, just return the original data

        if let (Some(_width), Some(_height)) = (max_width, max_height) {
            // TODO: Implement image resizing
            // This would resize the image to fit within the specified dimensions
        }

        Ok(image_data)
    }

    /// Generate thumbnail for images
    pub async fn generate_thumbnail(
        &self,
        conn: &mut PgConnection,
        image_id: &str,
        thumbnail_size: u32,
    ) -> Result<String, ImageError> {
        let metadata = self.get_image_metadata(conn, image_id)?;

        // Generate thumbnail key
        let thumbnail_key = format!("{}_thumb_{}", metadata.r2_key, thumbnail_size);

        // TODO: Implement thumbnail generation
        // 1. Download original image from R2
        // 2. Resize to thumbnail size
        // 3. Upload thumbnail to R2
        // 4. Return thumbnail key or URL

        Ok(thumbnail_key)
    }
}

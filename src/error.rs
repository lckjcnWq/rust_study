// 统一错误处理
#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Authentication required")]
    Unauthorized,
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        match self {
            AppError::Database(_) => 
                HttpResponse::InternalServerError().json("数据库错误"),
            AppError::Validation(msg) => 
                HttpResponse::BadRequest().json(msg),
            AppError::Unauthorized => 
                HttpResponse::Unauthorized().json("需要认证"),
        }
    }
} 
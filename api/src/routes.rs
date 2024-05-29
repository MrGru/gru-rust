
pub async fn hello() -> axum::Json<String> {
    axum::Json("Hello, World!".to_string())
}
use axum::response::{Html, IntoResponse};
use hyper::StatusCode;

/// Simple handler to return the default 404 error page.
pub async fn handle_404() -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(
            r#"
            <div>
                <h1>Nothing to see here.</h1>
            </div>
            "#,
        ),
    )
}

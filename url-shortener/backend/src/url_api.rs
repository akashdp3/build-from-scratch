use axum::http::StatusCode;
use axum::{extract::State, response::Json};
use std::sync::{Arc, Mutex};

use crate::api_response::{
    ApiError, ApiSuccess, CreateShortURLApiRequest, GetShortURLApiRequest, GetShortURLApiSuccess,
};
use crate::url_core::UrlService;

/** CREATE SHORT URL  **/
pub async fn create_short_url(
    State(urls): State<Arc<Mutex<UrlService>>>,
    Json(payload): Json<CreateShortURLApiRequest>,
) -> Result<ApiSuccess, ApiError> {
    let mut urls = urls.lock().expect("UrlService lock failed");
    let short_url = urls.create_short_url(payload.url);

    Ok(short_url.into())
}

/** GET SHORT URL  **/

pub async fn get_short_url(
    State(urls): State<Arc<Mutex<UrlService>>>,
    Json(payload): Json<GetShortURLApiRequest>,
) -> Result<ApiSuccess, ApiError> {
    let urls = urls.lock().expect("UrlService lock failed");
    let short_url = match urls.get_short_url(&payload.key) {
        Some(short_url) => short_url,
        None => {
            let err_msg = format!("Error getting short URL for key: {}", payload.key);
            println!("{}", err_msg);
            return Err(ApiError {
                status_code: axum::http::StatusCode::BAD_REQUEST,
                message: err_msg,
            });
        }
    };

    let data = serde_json::to_value(GetShortURLApiSuccess {
        original_url: short_url.original_url.clone(),
    })
    .expect("GetShortURLApiSuccess should always serialize to JSON");

    Ok(ApiSuccess {
        status_code: StatusCode::PERMANENT_REDIRECT,
        data,
        headers: vec![(
            "Cache-Control".to_string(),
            "public, max-age=31536000".to_string(),
        )],
    })
}

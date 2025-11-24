use crate::url_core::ShortUrl;
use axum::{
    Json,
    http::{HeaderName, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

pub struct ApiSuccess {
    pub status_code: StatusCode,
    pub data: Value,
    pub headers: Vec<(String, String)>,
}

pub struct ApiError {
    pub message: String,
    pub status_code: axum::http::StatusCode,
}

/** POST /url  **/
#[derive(Deserialize)]
pub struct CreateShortURLApiRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct CreateShortURLApiSuccess {
    shortened_url: String,
    original_url: String,
}

impl From<ShortUrl> for CreateShortURLApiSuccess {
    fn from(value: ShortUrl) -> Self {
        Self {
            shortened_url: format!("http://localhost:5173/{}", value.key),
            original_url: value.original_url,
        }
    }
}

impl From<CreateShortURLApiSuccess> for ApiSuccess {
    fn from(value: CreateShortURLApiSuccess) -> Self {
        let data = serde_json::to_value(value)
            .expect("CreateShortURLApiSuccess should always serialize to JSON");
        Self {
            status_code: StatusCode::OK,
            data,
            headers: Vec::new(),
        }
    }
}

impl From<ShortUrl> for ApiSuccess {
    fn from(value: ShortUrl) -> Self {
        CreateShortURLApiSuccess::from(value).into()
    }
}

impl IntoResponse for ApiSuccess {
    fn into_response(self) -> Response {
        let mut response = (self.status_code, Json(self.data)).into_response();
        for (name, value) in self.headers {
            let header_name = HeaderName::from_bytes(name.as_bytes())
                .expect("Invalid header name provided to ApiSuccess");
            let header_value =
                HeaderValue::from_str(&value).expect("Invalid header value provided to ApiSuccess");
            response.headers_mut().insert(header_name, header_value);
        }

        response
    }
}

/** Get /url **/
#[derive(Deserialize)]
pub struct GetShortURLApiRequest {
    pub key: String,
}

#[derive(Serialize)]
pub struct GetShortURLApiSuccess {
    pub original_url: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let body = json!({ "error": self.message });

        (self.status_code, Json(body)).into_response()
    }
}

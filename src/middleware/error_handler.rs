use actix_web::{
    middleware::ErrorHandlerResponse,
    http::header::{CONTENT_TYPE, HeaderValue},
    dev::ServiceResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ResponseMessage {
    detail: String,
}

pub fn default_handler<B>(mut res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let res = match res.response().error() {
        None => res.map_into_left_body(),
        Some(_) => {
            res.response_mut().headers_mut().insert(
                CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            let (req, res) = res.into_parts();
            let rm = ResponseMessage {
                detail: res.error().unwrap().to_string()
            };
            let body = serde_json::json!(&rm).to_string();
            let res = res.set_body(body).map_into_boxed_body();
            ServiceResponse::new(req, res).map_into_right_body()
        }
    };
    Ok(ErrorHandlerResponse::Response(res))
}
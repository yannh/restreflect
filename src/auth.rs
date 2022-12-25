use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};

#[utoipa::path(
    get,
    path = "/bearer",
    tag = "Auth",
    responses(
        (status = 200, description = "Successful authentication", content_type = "application/json"),
        (status = 401, description = "Unsuccessful authentication", content_type = "application/json")
    )
)]
/// Prompts the user for authorization using bearer authentication."
pub fn bearer(req: &Request) -> Result<Response, Error> {
    let unauthorized = Ok(Response::from_status(StatusCode::UNAUTHORIZED)
        .with_content_type(mime::APPLICATION_JSON));

    match req.get_header("authorization") {
        Some(auth) => {
            let token = auth.to_str().unwrap_or("").strip_prefix("Bearer ");
            if token.is_none() {
                return unauthorized;
            }

            let resp = json!({
                "authenticated": true,
                "token": token,
            });

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(to_string_pretty(&resp).unwrap()))
        },
        None => unauthorized,
    }
}

use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use uuid::Uuid;

#[utoipa::path(
    get,
    path = "/uuid",
    tag = "Dynamic data",
    responses(
        (status = 200, description = "A UUID4", content_type = "application/json"),
    )
)]
/// Return a UUID4.
pub fn uuid(req: &Request) -> Result<Response, Error> {
    let resp = json!({
        "uuid": Uuid::new_v4().to_string(),
    });

    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()));
}

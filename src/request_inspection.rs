use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use crate::utils::req_headers;

#[utoipa::path(
    get,
    path = "/user-agent",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The request’s User-Agent header.", content_type = "application/json")
    )
)]
/// Return the incoming requests's User-Agent header.
pub fn user_agent(req: &Request) -> Result<Response, Error> {
    let ua = req.get_header("user-agent").unwrap().to_str().unwrap();
    let resp = json!({
            "user-agent": ua
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()))
}

#[utoipa::path(
    get,
    path = "/ip",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The Requester's IP address", content_type = "application/json")
    )
)]
/// Returns the requester's IP Address.
pub fn ip(req: &Request) -> Result<Response, Error> {
    let resp = json!({
            "ip": req.get_client_ip_addr()
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()))
}

#[utoipa::path(
    get,
    path = "/headers",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The Request's headers", content_type = "application/json")
    )
)]
/// Return the incoming request's HTTP headers
pub fn headers(req: &Request) -> Result<Response, Error> {
    let resp = json!({
            "headers": req_headers(req),
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap_or_default()))
}

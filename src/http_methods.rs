use std::collections::HashMap;
use std::path::Path;
use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use regex::{Regex};
use utoipa::OpenApi;
use serde_json::{json, to_string_pretty};

fn rr_http_methods(req: Request) -> Result<Response, Error> {
    let headers: HashMap<&str, &str>= req.get_headers()
        .map(|m| (m.0.as_str(), m.1.to_str().unwrap_or("")))
        .collect();

    let resp = json!({
            "headers": headers,
            "origin": req.get_client_ip_addr(),
            "url": req.get_url_str()
        });

    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body(to_string_pretty(&resp).unwrap()))
}
#[utoipa::path(
    get,
    path = "/get",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's query parameters.", content_type = "application/json")
    )
)]
pub fn rr_get(req: Request) -> Result<Response, Error> {
    return rr_http_methods(req)
}

#[utoipa::path(
    post,
    path = "/post",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's POST parameters.", content_type = "application/json")
    )
)]
pub fn rr_post(req: Request) -> Result<Response, Error> {
    return rr_http_methods(req)
}

#[utoipa::path(
    put,
    path = "/put",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's PUT parameters.", content_type = "application/json")
    )
)]
pub fn rr_put(req: Request) -> Result<Response, Error> {
    return rr_http_methods(req)
}

#[utoipa::path(
    patch,
    path = "/patch",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's PATCH parameters.", content_type = "application/json")
    )
)]
pub fn rr_patch(req: Request) -> Result<Response, Error> {
    return rr_http_methods(req)
}

#[utoipa::path(
delete,
path = "/delete",
tag = "HTTP Methods",
responses(
(status = 200, description = "The request's DELETE parameters.", content_type = "application/json")
    )
)]
pub fn rr_delete(req: Request) -> Result<Response, Error> {
    return rr_http_methods(req)
}


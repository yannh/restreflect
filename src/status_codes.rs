use std::collections::HashMap;
use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use regex::Regex;

fn rr_http_statuses(req: Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/status/(\d{3})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
        let status = caps.unwrap().get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap_or(404));
        return Ok(Response::from_status(StatusCode::from_u16(status).unwrap_or(StatusCode::NOT_FOUND))
            .with_content_type(mime::TEXT_HTML_UTF_8))
    }

    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8));
}

#[utoipa::path(
    get,
    path = "/status/{codes}",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn get(req: Request) -> Result<Response, Error> {
    return rr_http_statuses(req)
}

#[utoipa::path(
    post,
    path = "/status/{codes}",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn post(req: Request) -> Result<Response, Error> {
    return rr_http_statuses(req)
}

#[utoipa::path(
    put,
    path = "/status/{codes}",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn put(req: Request) -> Result<Response, Error> {
    return rr_http_statuses(req)
}

#[utoipa::path(
    patch,
    path = "/status/{codes}",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]

/// Return status code or random status code if more than one is given
pub fn patch(req: Request) -> Result<Response, Error> {
    return rr_http_statuses(req)
}
#[utoipa::path(
    delete,
    path = "/status/{codes}",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn delete(req: Request) -> Result<Response, Error> {
    return rr_http_statuses(req)
}

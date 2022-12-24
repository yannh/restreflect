use std::path::Path;
use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use regex::{Regex};
use rust_embed::RustEmbed;
use std::ffi::OsStr;
use utoipa::OpenApi;
#[utoipa::path(
    get,
    path = "/html",
    tag = "Response formats",
    responses(
        (status = 200, description = "An HTML page.", content_type = "text/html")
    )
)]
/// Returns a simple HTML document.
pub fn html(req: Request) -> Result<Response, Error> {
    return crate::assets::serve(req);
}

#[utoipa::path(
    get,
    path = "/json",
    tag = "Response formats",
    responses(
        (status = 200, description = "A JSON document.", content_type = "application/json")
    )
)]
/// Returns a simple JSON document.
pub fn json(req: Request) -> Result<Response, Error> {
    return crate::assets::serve(req);
}

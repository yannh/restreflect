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
    return crate::assets::serve("html.html", mime::TEXT_HTML);
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
    return crate::assets::serve("json.json", mime::APPLICATION_JSON);
}

#[utoipa::path(
    get,
    path = "/robots.txt",
    tag = "Response formats",
    responses(
        (status = 200, description = "Robots file", content_type = "text/plain")
    )
)]
/// Returns some robots.txt rules.
pub fn robots_txt(req: Request) -> Result<Response, Error> {
    return crate::assets::serve("robots.txt", mime::TEXT_PLAIN);
}

#[utoipa::path(
    get,
    path = "/xml",
    tag = "Response formats",
    responses(
        (status = 200, description = "A XML document.", content_type = "application/xml")
    )
)]
/// Returns a simple XML document.
pub fn xml(req: Request) -> Result<Response, Error> {
    let mime_xml: mime::Mime = "application/xml".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM);
    return crate::assets::serve("robots.txt", mime_xml);
}

#[utoipa::path(
    get,
    path = "/deny",
    tag = "Response formats",
    responses(
        (status = 200, description = "Denied message", body=str, content_type = "text/plain")
    )
)]
/// Returns page denied by robots.txt rules.
pub fn deny(req: Request) -> Result<Response, Error> {
    return crate::assets::serve("deny.txt", mime::TEXT_PLAIN);
}

#[utoipa::path(
    get,
    path = "/encoding/utf8",
    tag = "Response formats",
    responses(
        (status = 200, description = "Encoded UTF-8 content.", body=str, content_type = "text/plain")
    )
)]
/// Returns a UTF-8 encoded body.
pub fn encoding_utf8(req: Request) -> Result<Response, Error> {
    return crate::assets::serve("utf8.txt", mime::TEXT_PLAIN);
}

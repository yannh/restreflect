use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use RESTReflect::req_to_json;
use deflate::{deflate_bytes, deflate_bytes_gzip};

#[utoipa::path(
    get,
    path = "/brotli",
    tag = "Response formats",
    responses(
    (status = 200, description = "Brotli-encoded data.", content_type = "application/json")
    )
)]
/// Returns Brotli-encoded data.
pub fn brotli(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let mut enc: Vec<u8> = vec!();
    let params = brotli::enc::BrotliEncoderParams::default();
    match brotli::BrotliCompress(&mut res.as_bytes(), &mut enc, &params) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "br")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/deflate",
    tag = "Response formats",
    responses(
        (status = 200, description = "Deflate-encoded data.", content_type = "application/json")
    )
)]
/// Returns Deflate-encoded data.
pub fn deflate(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let enc = deflate_bytes(res.as_bytes());
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "deflate")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/gzip",
    tag = "Response formats",
    responses(
        (status = 200, description = "GZip-encoded data.", content_type = "application/json")
    )
)]
/// Returns GZip-encoded data.
pub fn gzip(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let enc = deflate_bytes_gzip(res.as_bytes());
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "gzip")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/html",
    tag = "Response formats",
    responses(
        (status = 200, description = "An HTML page.", content_type = "text/html")
    )
)]
/// Returns a simple HTML document.
pub fn html(_: &Request) -> Result<Response, Error> {
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
pub fn json(_: &Request) -> Result<Response, Error> {
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
pub fn robots_txt(_: &Request) -> Result<Response, Error> {
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
pub fn xml(_: &Request) -> Result<Response, Error> {
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
pub fn deny(_: &Request) -> Result<Response, Error> {
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
pub fn encoding_utf8(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("utf8.txt", mime::TEXT_PLAIN);
}

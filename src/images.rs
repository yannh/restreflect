use fastly::{Error, mime, Request, Response};
use fastly::http::HeaderValue;
use crate::StatusCode;

#[utoipa::path(
    get,
    path = "/image/jpeg",
    tag = "Images",
    responses(
        (status = 200, description = "A JPEG image.", content_type = "image/jpeg")
    )
)]
/// Returns a simple JPEG image.
pub fn jpeg(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("jpeg.jpeg", mime::IMAGE_JPEG);
}

#[utoipa::path(
    get,
    path = "/image/png",
    tag = "Images",
    responses(
        (status = 200, description = "A PNG image.", content_type = "image/png")
    )
)]
/// Returns a simple PNG image.
pub fn png(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("png.png", mime::IMAGE_PNG);
}

#[utoipa::path(
    get,
    path = "/image/svg",
    tag = "Images",
    responses(
        (status = 200, description = "A SVG image.", content_type = "image/png")
    )
)]
/// Returns a simple SVG image.
pub fn svg(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("svg.svg", mime::IMAGE_SVG);
}

#[utoipa::path(
    get,
    path = "/image/webp",
    tag = "Images",
    responses(
        (status = 200, description = "A WEBP image.", content_type = "image/webp")
    )
)]
/// Returns a simple WEBP image.
pub fn webp(_: &Request) -> Result<Response, Error> {
    let mime_webp: mime::Mime = "image/webp".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM);
    crate::assets::serve("webp.webp", mime_webp)
}

#[utoipa::path(
    get,
    path = "/image",
    tag = "Images",
    responses(
        (status = 200, description = "An image.", content_type = "image/webp")
    )
)]
/// Returns a simple image of the type suggest by the Accept header.
pub fn image(req: &Request) -> Result<Response, Error> {
    // reproduced logic from https://github.com/postmanlabs/httpbin/blob/f8ec666b4d1b654e4ff6aedd356f510dcac09f83/httpbin/core.py#L1645
    let default = &HeaderValue::from_static("image/png");
    let accept = req
        .get_header("accept")
        .unwrap_or(default)
        .to_str()
        .unwrap_or_default();
    if accept.contains("image/webp") {
        return webp(req)
    }
    if accept.contains("image/svg+xml") {
        return svg(req)
    }
    if accept.contains("image/jpeg") {
        return jpeg(req)
    }
    if accept.contains("image/png") || accept.contains("image/*") {
        return png(req)
    }
    Ok(Response::from_status(StatusCode::NOT_ACCEPTABLE)
        .with_content_type(mime::APPLICATION_JSON))
}

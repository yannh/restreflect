use fastly::{Error, mime, Request, Response};

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
    return crate::assets::serve("webp.webp", mime_webp);
}

use std::path::Path;
use fastly::http::StatusCode;
use fastly::{Error, mime, Response};
use rust_embed::RustEmbed;
use std::ffi::OsStr;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Asset;

pub fn file_mimetype(filename: &str, default: mime::Mime) -> mime::Mime {
    let extension = Path::new(filename)
        .extension()
        .and_then(OsStr::to_str)
        .map(|s| s.to_lowercase());

    let mime_webp: mime::Mime = "image/webp".parse().unwrap_or(default.clone());
    match extension {
        Some(ext) => match ext.as_str() {
            "css" => mime::TEXT_CSS_UTF_8,
            "gif" => mime::IMAGE_GIF,
            "html" | "htm" => mime::TEXT_HTML_UTF_8,
            "jpeg" | "jpg" => mime::IMAGE_JPEG,
            "png" => mime::IMAGE_PNG,
            "js" => mime::TEXT_JAVASCRIPT,
            "json" => mime::APPLICATION_JSON,
            "svg" => mime::IMAGE_SVG,
            "txt" => mime::TEXT_PLAIN,
            "webp" => mime_webp, // webp not supported https://github.com/hyperium/mime/pull/129
            // unfortunately the mime library is unmaintained
            "xml" => mime::TEXT_XML,
            _ => default,
        },
        _ => default,
    }
}

pub fn serve(path: &str, mt: mime::Mime) -> Result<Response, Error> {
    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    return match Asset::get(path) {
        Some(asset) => Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.data.as_ref())
            .with_content_type(file_mimetype(path, mt))),

        None => not_found,
    }
}

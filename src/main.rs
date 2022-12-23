use std::collections::HashMap;
use std::path::Path;
use fastly::http::{StatusCode};
use fastly::{mime, Error, Request, Response};
use regex::Regex;
use serde_json::json;
use rust_embed::RustEmbed;
use std::ffi::OsStr;

#[derive(RustEmbed)]
#[folder = "assets/"]
struct Asset;


fn file_mimetype(filename: &str, default: mime::Mime) -> mime::Mime {
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
            "svg" => mime::IMAGE_SVG,
            "webp" => mime_webp, // webp not supported https://github.com/hyperium/mime/pull/129
                                 // unfortunately the mime library is unmaintained
            _ => default,
        },
        _ => default,
    }
}


#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    let caps = Regex::new(r"/status/(\d{3})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
       let status = caps.unwrap().get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap_or(404));
       return Ok(Response::from_status(StatusCode::from_u16(status).unwrap_or(StatusCode::NOT_FOUND))
           .with_content_type(mime::TEXT_HTML_UTF_8))
    }

    let http_methods_paths = ["/delete", "/get", "/patch", "/post", "/put"];
    if http_methods_paths.contains(&req.get_path()) {
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
            .with_body(resp.to_string()))
    }

    let images_paths = ["/image/jpeg", "/image/png", "/image/svg", "/image/webp"];
    if images_paths.contains(&req.get_path()) {
        let img_path = match req.get_path() {
            "/image/jpeg" => "jpeg.jpeg",
            "/image/png" => "png.png",
            "/image/svg" => "svg.svg",
            "/image/webp" => "webp.webp",
            _ => ""
        };

        return match Asset::get(img_path) {
            Some(asset) => Ok(Response::from_status(StatusCode::OK)
                .with_body_octet_stream(asset.data.as_ref())
                .with_content_type(file_mimetype(img_path, mime::APPLICATION_OCTET_STREAM))),

            None => not_found,
        }
    }

    if req.get_path() == "/user-agent" {
        let ua = req.get_header("user-agent").unwrap().to_str().unwrap_or("");
        let resp = json!({
            "user-agent": ua
        });

        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(resp.to_string()))
    }

    if req.get_path() == "/ip" {
        let resp = json!({
            "ip": req.get_client_ip_addr()
        });

        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(resp.to_string()))
    }

    if req.get_path() == "/" || req.get_path() == "/index" || req.get_path() == "index.html" {
        return match Asset::get("index.html") {
            Some(asset) => Ok(Response::from_status(StatusCode::OK)
                .with_body_octet_stream(asset.data.as_ref())
                .with_content_type(mime::TEXT_HTML_UTF_8)),

            None => not_found,
        }
    }

    return not_found
}

mod http_methods;
mod request_inspection;
mod status_codes;

use std::path::Path;
use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use regex::{Regex};
use rust_embed::RustEmbed;
use std::ffi::OsStr;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    http_methods::delete, http_methods::get, http_methods::put, http_methods::post, http_methods::patch,
    request_inspection::user_agent, request_inspection::ip,
    status_codes::get, status_codes::post, status_codes::put, status_codes::patch, status_codes::delete,
  ),
  tags(
    (name = "HTTP Methods", description = "Testing different HTTP verbs"),
    (name = "Request inspection", description = "Inspect the request data"),
    (name = "Status codes", description = "Generates responses with given status code"),
  ),
)]
struct ApiDoc;

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



fn rr_serve_asset(req: Request) -> Result<Response, Error> {
    let path = match req.get_path() {
        "/deny" => "robots.txt",
        "/json" => "json.json",
        "/html" => "html.html",
        "/robots.txt" => "robots.txt",
        "/encoding/utf8" => "utf8.txt",
        "/xml" => "xml.xml",
        "/image/jpeg" => "jpeg.jpeg",
        "/image/png" => "png.png",
        "/image/svg" => "svg.svg",
        "/image/webp" => "webp.webp",
        _ => ""
    };

    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    return match Asset::get(path) {
        Some(asset) => Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.data.as_ref())
            .with_content_type(file_mimetype(path, mime::APPLICATION_OCTET_STREAM))),

        None => not_found,
    }
}

fn rr_index(req: Request) -> Result<Response, Error> {
    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    return match Asset::get("index.html") {
        Some(asset) => Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.data.as_ref())
            .with_content_type(mime::TEXT_HTML_UTF_8)),

        None => not_found,
    }
}



fn rr_swagger(req: Request) -> Result<Response, Error> {
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(ApiDoc::openapi().to_pretty_json().unwrap()));
}

fn route(routes:Vec<(Method, Regex, fn(Request) -> Result<Response, Error>)>, req: Request) -> Result<Response, Error>{
   for (method, r, cb) in routes {
       if method == req.get_method() && r.is_match(req.get_path()) {
           return cb(req)
       }
   }

   return Ok(Response::from_status(StatusCode::NOT_FOUND)
     .with_content_type(mime::TEXT_HTML_UTF_8));
}

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let path = match req.get_path() {
        "/" => "/index.html",
        "/index" => "/index.html",
        "/index.html" => "/index.html",
        _ => req.get_path(),
    };

    let mut p = path.to_owned();
    p.insert_str(0, "site");
    let asset = Asset::get(p.as_str());
    if asset.is_some() {
        return Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.unwrap().data.as_ref())
            .with_content_type(file_mimetype(path, mime::APPLICATION_OCTET_STREAM)));
    }

    type RequestHandler = fn(Request) -> Result<Response, Error>;
    let mut routes: Vec<(Method, Regex, RequestHandler)> = vec![
        (Method::GET, Regex::new(r"/(index(\.html)?)?$").unwrap(), rr_index),
        (Method::GET, Regex::new(r"^/status/(\d{3})$").unwrap(), status_codes::get),
        (Method::POST, Regex::new(r"^/status/(\d{3})$").unwrap(),status_codes::post),
        (Method::PUT, Regex::new(r"^/status/(\d{3})$").unwrap(), status_codes::put),
        (Method::PATCH, Regex::new(r"^/status/(\d{3})$").unwrap(), status_codes::patch),
        (Method::DELETE, Regex::new(r"^/status/(\d{3})$").unwrap(), status_codes::delete),
        (Method::GET, Regex::new(r"^/delete$").unwrap(), http_methods::delete),
        (Method::GET, Regex::new(r"^/get$").unwrap(), http_methods::get),
        (Method::PATCH, Regex::new(r"^/patch$").unwrap(), http_methods::patch),
        (Method::POST, Regex::new(r"^/post$").unwrap(), http_methods::post),
        (Method::PUT, Regex::new(r"^/put$").unwrap(), http_methods::put),
        (Method::GET, Regex::new(r"^/image/(jpeg|png|svg|webp)$").unwrap(), rr_serve_asset),
        (Method::GET, Regex::new(r"/(html|json|robots\.txt|xml|deny|utf8)$").unwrap(), rr_serve_asset),
        (Method::GET, Regex::new(r"/user-agent$").unwrap(), request_inspection::user_agent),
        (Method::GET, Regex::new(r"/ip$").unwrap(), request_inspection::ip),
        (Method::GET, Regex::new(r"/swagger\.json$").unwrap(), rr_swagger),
    ];
    return route(routes, req);
}

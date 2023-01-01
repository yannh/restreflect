mod assets;
mod auth;
mod dynamic_data;
mod http_methods;
mod images;
mod redirects;
mod request_inspection;
mod response_inspection;
mod response_formats;
mod status_codes;
mod lib;

use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use regex::{Regex};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
  paths(
    auth::bearer, auth::basic_auth,
    dynamic_data::uuid, dynamic_data::delay_get, dynamic_data::delay_post, dynamic_data::base64,
    dynamic_data::bytes,
    http_methods::delete, http_methods::get, http_methods::put, http_methods::post, http_methods::patch,
    images::image, images::jpeg, images::png, images::svg, images::webp,
    redirects::relative_redirect, redirects::redirect,
    request_inspection::user_agent, request_inspection::ip, request_inspection::headers,
    response_inspection::response_headers_get, response_inspection::response_headers_post,
    response_inspection::etag,
    response_formats::html, response_formats::json, response_formats::xml, response_formats::encoding_utf8,
    response_formats::deny, response_formats::robots_txt, response_formats::brotli,
    response_formats::deflate, response_formats::gzip,
    status_codes::get, status_codes::post, status_codes::put, status_codes::patch, status_codes::delete,
  ),
  tags(
    (name = "Auth", description = "Auth methods"),
    (name = "Dynamic data", description = "Generates random and dynamic data"),
    (name = "HTTP Methods", description = "Testing different HTTP verbs"),
    (name = "Redirects", description = "Returns different redirect responses"),
    (name = "Request inspection", description = "Inspect the request data"),
    (name = "Response inspection", description = "Inspect the response data like caching and headers"),
    (name = "Response formats", description = "Returns responses in different data formats"),
    (name = "Status codes", description = "Generates responses with given status code"),
  ),
)]
struct ApiDoc;

enum ReqHandler {
    MutHandler (fn(&mut Request) -> Result<Response, Error>),
    Handler(fn(&Request) -> Result<Response, Error>),
}

fn rr_index(_: &Request) -> Result<Response, Error> {
    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    return match crate::assets::Asset::get("index.html") {
        Some(asset) => Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.data.as_ref())
            .with_content_type(mime::TEXT_HTML_UTF_8)),

        None => not_found,
    }
}

fn rr_swagger(_: &Request) -> Result<Response, Error> {
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(ApiDoc::openapi().to_pretty_json().unwrap_or_default()));
}

fn route(routes:Vec<(Method, Regex, ReqHandler)>, req: &mut Request) -> Result<Response, Error>{
   for (method, r, handler) in routes {
       if method == req.get_method() && r.is_match(req.get_path()) {
           return match handler {
               ReqHandler::MutHandler(cb) => cb(req),
               ReqHandler::Handler(cb) => cb(req),
           };
       }
   }

   return Ok(Response::from_status(StatusCode::NOT_FOUND)
     .with_content_type(mime::TEXT_HTML_UTF_8));
}

#[fastly::main]
fn main(mut req: Request) -> Result<Response, Error> {
    let path = match req.get_path() {
        "/" | "/index" | "/index.html" => "/index.html",
        _ => req.get_path(),
    };

    if let Some(asset) = assets::Asset::get(format!("swagger-ui{path}").as_str()) {
        return Ok(Response::from_status(StatusCode::OK)
            .with_body_octet_stream(asset.data.as_ref())
            .with_content_type(assets::file_mimetype(path, mime::APPLICATION_OCTET_STREAM)));
    }

    use ReqHandler::*;
    let routes: Vec<(Method, Regex, ReqHandler)> = vec![
        (Method::GET, Regex::new(r"/(index.html)?$")?, Handler(rr_index)),
        (Method::GET, Regex::new(r"/swagger\.json$")?, Handler(rr_swagger)),
        (Method::GET, Regex::new(r"^/status/((\d{3},?)+)$")?, Handler(status_codes::get)),
        (Method::POST, Regex::new(r"^/status/(\d{3})$")?, MutHandler(status_codes::post)),
        (Method::PUT, Regex::new(r"^/status/(\d{3})$")?, MutHandler(status_codes::put)),
        (Method::PATCH, Regex::new(r"^/status/(\d{3})$")?, MutHandler(status_codes::patch)),
        (Method::DELETE, Regex::new(r"^/status/(\d{3})$")?, Handler(status_codes::delete)),
        (Method::PATCH, Regex::new(r"^/patch$")?, MutHandler(http_methods::patch)),
        (Method::POST, Regex::new(r"^/post$")?, MutHandler(http_methods::post)),
        (Method::PUT, Regex::new(r"^/put$")?, MutHandler(http_methods::put)),
        (Method::GET, Regex::new(r"^/delete$")?, Handler(http_methods::delete)),
        (Method::GET, Regex::new(r"^/get$")?, Handler(http_methods::get)),
        (Method::GET, Regex::new(r"^/image$")?, Handler(images::image)),
        (Method::GET, Regex::new(r"^/image/jpeg$")?, Handler(images::jpeg)),
        (Method::GET, Regex::new(r"^/image/png$")?, Handler(images::png)),
        (Method::GET, Regex::new(r"^/image/svg$")?, Handler(images::svg)),
        (Method::GET, Regex::new(r"^/image/webp$")?, Handler(images::webp)),
        (Method::GET, Regex::new(r"^/deflate$")?, Handler(response_formats::deflate)),
        (Method::GET, Regex::new(r"^/brotli$")?, Handler(response_formats::brotli)),
        (Method::GET, Regex::new(r"^/gzip$")?, Handler(response_formats::gzip)),
        (Method::GET, Regex::new(r"^/html$")?, Handler(response_formats::html)),
        (Method::GET, Regex::new(r"^/json$")?, Handler(response_formats::json)),
        (Method::GET, Regex::new(r"^/robots\.txt$")?, Handler(response_formats::robots_txt)),
        (Method::GET, Regex::new(r"^/xml$")?, Handler(response_formats::xml)),
        (Method::GET, Regex::new(r"^/deny$")?, Handler(response_formats::deny)),
        (Method::GET, Regex::new(r"^/utf8$")?, Handler(response_formats::encoding_utf8)),
        (Method::GET, Regex::new(r"^/user-agent$")?, Handler(request_inspection::user_agent)),
        (Method::GET, Regex::new(r"^/ip$")?, Handler(request_inspection::ip)),
        (Method::GET, Regex::new(r"^/basic-auth/(\w+)/(\w+)$")?, Handler(auth::basic_auth)),
        (Method::GET, Regex::new(r"^/bearer$")?, Handler(auth::bearer)),
        (Method::GET, Regex::new(r"^/base64/([A-Za-z0-9+/=]{1,4096})$")?, Handler(dynamic_data::base64)),
        (Method::GET, Regex::new(r"^/bytes/(\d{1,5})$")?, Handler(dynamic_data::bytes)),
        (Method::GET, Regex::new(r"^/uuid$")?, Handler(dynamic_data::uuid)),
        (Method::GET, Regex::new(r"^/delay/(\d{1,2})$")?, Handler(dynamic_data::delay_get)),
        (Method::POST, Regex::new(r"^/delay/(\d{1,2})$")?, MutHandler(dynamic_data::delay_post)),
        (Method::GET, Regex::new(r"^/headers$")?, Handler(request_inspection::headers)),
        (Method::GET, Regex::new(r"^/etag/(\w+)$")?, Handler(response_inspection::etag)),
        (Method::GET, Regex::new(r"^/response-headers$")?, Handler(response_inspection::response_headers_get)),
        (Method::POST, Regex::new(r"^/response-headers$")?, Handler(response_inspection::response_headers_post)),
        (Method::GET, Regex::new(r"^/relative-redirect/(\d{1})$")?, Handler(redirects::relative_redirect)),
        (Method::GET, Regex::new(r"^/redirect/(\d{1})$")?, Handler(redirects::redirect)),
    ];

    route(routes, &mut req).map (|resp|
        resp
            .with_header("access-control-allow-origin", "*")
            .with_header("access-control-allow-credentials", "true")
    )
}

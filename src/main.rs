use std::collections::HashMap;
use fastly::http::{HeaderName, HeaderValue, StatusCode};
use fastly::{mime, Error, Request, Response};
use regex::Regex;
use serde_json::json;

#[fastly::main]
fn main(req: Request) -> Result<Response, Error> {
    let not_found = Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8)
        .with_body("E_NOTFOUND"));

    let caps = Regex::new(r"/status/(\d{3})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
       let status = caps.unwrap().get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap());
       return Ok(Response::from_status(StatusCode::from_u16(status).unwrap())
           .with_content_type(mime::TEXT_HTML_UTF_8))
    }

    let http_methods_paths = ["/delete", "/get", "/patch", "/post", "/put"];
    if http_methods_paths.contains(&req.get_path()) {
        let h: HashMap<&str, &str>= req.get_headers()
            .map(|m| (m.0.as_str(), m.1.to_str().unwrap()))
            .collect();

        let resp = json!({
            "headers": h
        });

        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::TEXT_HTML_UTF_8)
            .with_body(resp.to_string()))
    }

    return not_found
}

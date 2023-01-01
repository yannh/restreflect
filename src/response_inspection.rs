use fastly::http::{HeaderValue, StatusCode};
use regex::Regex;
use fastly::{Error, mime, Request, Response};
use std::collections::HashMap;

#[utoipa::path(
    get,
    path = "/response-headers",
    tag = "Response inspection",
    params(
        ("freeform" = String, Query, description = "Query string with parameters to return as headers."),
    ),
    responses(
        (status = 200, description = "Response headers", content_type = "application/json")
    )
)]
/// Returns a set of response headers from the query string
pub fn response_headers_get(req: &Request) -> Result<Response, Error> {
    let arg_pairs: Vec<(String, String)> = req.get_query()?;
    let args: HashMap<&str, &str> = arg_pairs.iter().map(|m| (m.0.as_str(), m.1.as_str()))
        .collect();

    let mut resp =  Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON);

    for (k, v) in args {
        resp = resp.with_header(k, v);
    }

    return Ok(resp)
}


#[utoipa::path(
    post,
    path = "/response-headers",
    tag = "Response inspection",
    params(
        ("freeform" = String, Query, description = "Query string with parameters to return as headers."),
    ),
    responses(
        (status = 200, description = "Response headers", content_type = "application/json")
    )
)]
/// Returns a set of response headers from the query string
pub fn response_headers_post(req: &Request) -> Result<Response, Error> {
    return response_headers_get(req)
}

#[utoipa::path(
    get,
    path = "/etag",
    tag = "Response inspection",
    params(
        ("if-none-match" = String, Header),
        ("if-match" = String, Header),
    ),
    responses(
        (status = 200, description = "Response headers", content_type = "application/json"),
        (status = 412, description = "match", content_type = "application/json")
    )
)]
/// Assumes the resource has the given etag and responds to If-None-Match and If-Match headers appropriately.
pub fn etag(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/etag/(\w+)$")?
        .captures(req.get_path());
    if caps.is_some() {
        let etag = caps.unwrap().get(1).map_or("404", |m| m.as_str());
        let d = HeaderValue::from_static("");
        let if_none_match: Vec<&str> = req
            .get_header("if-none-match")
            .unwrap_or(&d)
            .to_str()
            .unwrap_or_default()
            .split(",")
            .collect();
        if if_none_match.contains(&etag) || if_none_match.contains(&"*"){
            return Ok(Response::from_status(StatusCode::NOT_MODIFIED)
                .with_content_type(mime::APPLICATION_JSON));
        }

        let if_match: Vec<&str> = req
            .get_header("if-none-match")
            .unwrap_or(&d)
            .to_str()
            .unwrap_or_default()
            .split(",")
            .collect();
        if !if_match.contains(&etag) && !if_match.contains(&"*"){
            return Ok(Response::from_status(StatusCode::PRECONDITION_FAILED)
                .with_content_type(mime::APPLICATION_JSON));
        }

        return Ok(Response::from_status(StatusCode::OK)
            .with_header("ETag", etag)
            .with_content_type(mime::APPLICATION_JSON));
    }

    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::APPLICATION_JSON));
}

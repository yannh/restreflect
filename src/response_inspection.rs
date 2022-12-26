use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
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
pub fn response_headers(req: &Request) -> Result<Response, Error> {
    let arg_pairs: Vec<(String, String)> = req.get_query().unwrap();
    let args: HashMap<&str, &str> = arg_pairs.iter().map(|m| (m.0.as_str(), m.1.as_str()))
        .collect();

    let mut resp =  Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON);

    for (k, v) in args {
        resp = resp.with_header(k, v);
    }

    return Ok(resp)
}

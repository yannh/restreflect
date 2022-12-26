use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use uuid::Uuid;
use std::{thread, time};
use regex::Regex;
use crate::lib::req_to_json;

#[utoipa::path(
    get,
    path = "/uuid",
    tag = "Dynamic data",
    responses(
        (status = 200, description = "A UUID4", content_type = "application/json"),
    )
)]
/// Return a UUID4.
pub fn uuid(req: &Request) -> Result<Response, Error> {
    let resp = json!({
        "uuid": Uuid::new_v4().to_string(),
    });
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()));
}

#[utoipa::path(
    get,
    path = "/delay/{n]",
    tag = "Dynamic data",
    responses(
        (status = 200, description = "A delayed response", content_type = "application/json"),
    )
)]
/// Returns a delayed response (max 10s)
pub fn delay(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/delay/(\d{1,2})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
        let mut n = caps.unwrap().get(1).map_or(404, |m| m.as_str().parse::<u64>().unwrap_or(404));
        if n > 10 {
            n = 10;
        }
        let d = time::Duration::from_secs(n);
        thread::sleep(d);
        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_JSON)
            .with_body(req_to_json(req)));
    }

    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML))
}

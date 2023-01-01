use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use uuid::Uuid;
use std::{thread, time};
use regex::Regex;
use base64::decode;
use crate::lib::{req_to_json, req_with_body_to_json};

#[utoipa::path(
    get,
    path = "/uuid",
    tag = "Dynamic data",
    responses(
        (status = 200, description = "A UUID4", content_type = "application/json"),
    )
)]
/// Return a UUID4.
pub fn uuid(_: &Request) -> Result<Response, Error> {
    let resp = json!({
        "uuid": Uuid::new_v4().to_string(),
    });
    let ten_millis = time::Duration::from_millis(10);
    thread::sleep(ten_millis);
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap_or_default()));
}

#[utoipa::path(
    get,
    path = "/base64/{value}",
    tag = "Dynamic data",
    params(
        ("value" = u16, Path, description = "String in base64 to decode."),
    ),
    responses(
        (status = 200, description = "Decoded base64 content.", content_type = "text/html"),
    )
)]
/// Decodes base64-encoded string.
pub fn base64(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/base64/([A-Za-z0-9+/=]{1,4096})$")?
        .captures(req.get_path());
    if caps.is_some() {
        let b64 = caps.unwrap().get(1).map(|m| m.as_str().as_bytes()).unwrap();
        return match decode(&b64) {
            Ok(decoded) => Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(decoded)),
            Err(_) => Ok(Response::from_status(StatusCode::BAD_REQUEST)
                .with_content_type(mime::TEXT_PLAIN)
                .with_body("Provided data not in base64 format. Try SFRUUEJJTiBpcyBhd2Vzb21l")),
        }
    }

    return Ok(Response::from_status(StatusCode::BAD_REQUEST)
        .with_content_type(mime::TEXT_HTML)
        .with_body("Could not extract base64 data"));
}

pub fn delay(req: &Request, body: String) -> Result<Response, Error> {
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
            .with_body(body));
    }

    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML))
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
pub fn delay_get(req: &Request) -> Result<Response, Error> {
    return delay(req, req_to_json(req));
}

#[utoipa::path(
    post,
    path = "/delay/{n]",
    tag = "Dynamic data",
    responses(
        (status = 200, description = "A delayed response", content_type = "application/json"),
    )
)]
/// Returns a delayed response (max 10s)
pub fn delay_post(req: &mut Request) -> Result<Response, Error> {
    let body = req_with_body_to_json(req);
    return delay(req, body);
}

#[utoipa::path(
    get,
    path = "/bytes/{n}",
    tag = "Dynamic data",
    params(
        ("n" = u32, Path, description = "Number of bytes to return. Max: 99999"),
    ),
    responses(
        (status = 200, description = "Bytes.", content_type = "application/octet-stream"),
    )
)]
/// Returns n random bytes
pub fn bytes(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/bytes/(\d{1,5})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
        let n = caps.unwrap().get(1).map_or(100, |m| m.as_str().parse::<usize>().unwrap_or(100));
        let mut resp:Vec<u8> = vec![0u8; n];
        getrandom::getrandom(&mut resp)?;
        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_OCTET_STREAM)
            .with_body_octet_stream(&resp));
    }

    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::APPLICATION_OCTET_STREAM))
}

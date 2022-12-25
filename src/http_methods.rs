use std::collections::HashMap;
use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use RESTReflect::{req_to_json, req_with_body_to_json};

fn http_methods(req: &Request) -> Result<Response, Error> {
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(req_to_json(req)))
}

fn http_methods_mut(req: &mut Request) -> Result<Response, Error> {
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(req_with_body_to_json(req)))
}

#[utoipa::path(
    get,
    path = "/get",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's query parameters.", content_type = "application/json")
    )
)]
/// The request's query parameter
pub fn get(req: &Request) -> Result<Response, Error> {
    return http_methods(req)
}

#[utoipa::path(
    post,
    path = "/post",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's POST parameters.", content_type = "application/json")
    )
)]
/// The request's POST parameter
pub fn post(req: &mut Request) -> Result<Response, Error> {
    return http_methods_mut(req)
}

#[utoipa::path(
    put,
    path = "/put",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's PUT parameters.", content_type = "application/json")
    )
)]
/// The request's PUT parameter
pub fn put(req: &mut Request) -> Result<Response, Error> {
    return http_methods(req)
}

#[utoipa::path(
    patch,
    path = "/patch",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's PATCH parameters.", content_type = "application/json")
    )
)]
/// The request's PATCH parameter
pub fn patch(req: &mut Request) -> Result<Response, Error> {
    return http_methods(req)
}

#[utoipa::path(
    delete,
    path = "/delete",
    tag = "HTTP Methods",
    responses(
        (status = 200, description = "The request's DELETE parameters.", content_type = "application/json")
    )
)]
/// The request's DELETE parameter
pub fn delete(req: &Request) -> Result<Response, Error> {
    return http_methods(req)
}


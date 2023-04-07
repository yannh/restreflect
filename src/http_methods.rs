use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use crate::utils::{req_to_json, req_with_body_to_json};

fn http_methods(req: &Request) -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(req_to_json(req)))
}

fn http_methods_mut(req: &mut Request) -> Result<Response, Error> {
    Ok(Response::from_status(StatusCode::OK)
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
    http_methods(req)
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
    http_methods_mut(req)
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
    http_methods(req)
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
    http_methods(req)
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
    http_methods(req)
}



#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_get() {
        let req = &Request::from_client()
            .with_path("/get");
        let resp = get(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
    }

    #[test]
    fn test_get_with_parameters() {
        let req = &Request::from_client()
            .with_path("/get")
            .with_query_str("foo=bar&fud=baz");
        let resp = get(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));

        let body = resp.into_body_str();
        let v: Value = serde_json::from_str(body.as_str()).unwrap();
        assert_eq!(v["args"]["foo"], "bar");
        assert_eq!(v["args"]["fud"], "baz");
        assert_eq!(v["url"], "http://example.com/get?foo=bar&fud=baz");
    }
}

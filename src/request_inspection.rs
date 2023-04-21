use fastly::http::{StatusCode, Version};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use serde::{Deserialize};
use crate::utils::req_headers;

#[utoipa::path(
    get,
    path = "/user-agent",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The request’s User-Agent header.", content_type = "application/json")
    )
)]
/// Return the incoming requests's User-Agent header.
pub fn user_agent(req: &Request) -> Result<Response, Error> {
    let ua = req.get_header("user-agent").unwrap().to_str().unwrap();
    let resp = json!({
            "user-agent": ua
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()))
}

#[utoipa::path(
    get,
    path = "/ip",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The Requester's IP address", content_type = "application/json")
    )
)]
/// Returns the requester's IP Address.
pub fn ip(req: &Request) -> Result<Response, Error> {
    let resp = json!({
            "ip": req.get_client_ip_addr()
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap()))
}

#[utoipa::path(
    get,
    path = "/headers",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The Request's headers", content_type = "application/json")
    )
)]
/// Return the incoming request's HTTP headers
pub fn headers(req: &Request) -> Result<Response, Error> {
    let resp = json!({
            "headers": req_headers(req),
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap_or_default()))
}

#[utoipa::path(
    get,
    path = "/http-version",
    tag = "Request inspection",
    responses(
        (status = 200, description = "The requests HTTP version", content_type = "application/json")
    )
)]
/// Return the incoming request's HTTP headers
pub fn http_version(req: &Request) -> Result<Response, Error> {
    let resp = json!({
            "http_version": match req.get_version() {
        Version::HTTP_09 => "0.9",
        Version::HTTP_10 => "1.0",
        Version::HTTP_11 => "1.1",
        Version::HTTP_2 => "2",
        Version::HTTP_3 => "3",
        _ => "unknown",
    }
        });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap_or_default()))
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::*;

    #[test]
    fn test_user_agent() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct SerdeBody {
            #[serde(alias = "user-agent")]
            useragent: String,
        }

        let req = &Request::from_client()
            .with_header("user-agent", "Microsoft Explorer 6")
            .with_path("/user-agent");
        let resp = user_agent(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));

        let body = resp.into_body_str();
        let m: SerdeBody = serde_json::from_str(body.as_str()).unwrap();
        let expect = SerdeBody {
            useragent: String::from("Microsoft Explorer 6")
        };
        assert_eq!(m, expect);
    }

    #[test]
    fn test_ip_success() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct SerdeBody {
            ip: String,
        }

        let req = &Request::from_client()
            .with_path("/ip");
        let resp = ip(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));

        let body = resp.into_body_str();
        let m: SerdeBody = serde_json::from_str(body.as_str()).unwrap();
        let expect = SerdeBody {
            ip: String::from("127.0.0.1")
        };
        assert_eq!(m, expect);
    }

    #[test]
    fn test_headers_success() {
        #[derive(Debug, Deserialize, PartialEq)]
        struct SerdeBody {
            headers: HashMap<String, String>,
        }

        let req = &Request::from_client()
            .with_header("foo", "bar")
            .with_header("bee", "baz")
            .with_path("/headers");
        let resp = headers(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));

        let body = resp.into_body_str();
        let m: SerdeBody = serde_json::from_str(body.as_str()).unwrap();
        let expect = SerdeBody {
            headers: [
                (String::from("foo"), String::from("bar")),
                (String::from("bee"), String::from("baz"))
            ].iter().cloned().collect()
        };
        assert_eq!(m, expect);
    }
}

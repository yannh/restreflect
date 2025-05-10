use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
use regex::Regex;
use base64::{Engine as _, engine::general_purpose};

#[utoipa::path(
    get,
    path = "/basic-auth/{user}/{passwd}",
    tag = "Auth",
    params(
        ("user" = String, Path),
        ("passwd" = String, Path),
    ),
    responses(
        (status = 200, description = "Successful authentication", content_type = "application/json"),
        (status = 401, description = "Unsuccessful authentication", content_type = "application/json")
    )
)]
/// Prompts the user for authorization using HTTP Basic Auth
pub fn basic_auth(req: &Request) -> Result<Response, Error> {
    let unauthorized = Ok(Response::from_status(StatusCode::UNAUTHORIZED)
        .with_content_type(mime::APPLICATION_JSON));

    let authorization =  req.get_header("authorization");
    if authorization.is_none() {
        return Ok(Response::from_status(StatusCode::UNAUTHORIZED)
            .with_content_type(mime::APPLICATION_JSON)
            .with_header("www-authenticate", "Basic Realm=\"Fake Realm\""));
    }

    let enc_basic_auth = authorization.unwrap().to_str().unwrap_or_default().strip_prefix("Basic ").unwrap_or_default();
    let dec_basic_auth = String::from_utf8(general_purpose::STANDARD.decode(enc_basic_auth).unwrap_or(vec![])).unwrap_or_default();
    let credentials: Vec<&str> = dec_basic_auth.split(":").collect();
    if credentials.len() != 2 {
        return unauthorized;
    }
    let (given_user, given_pwd) = (credentials[0], credentials[1]);

    let caps = Regex::new(r"/basic-auth/(\w+)/(\w+)$")?
        .captures(req.get_path());
    if caps.is_none() {
        return unauthorized;
    }

    let c = caps.unwrap();
    let user = c.get(1).unwrap().as_str();
    let pwd = c.get(2).unwrap().as_str();

    if given_user == user && given_pwd == pwd {
        return Ok(Response::from_status(StatusCode::OK)
            .with_content_type(mime::APPLICATION_JSON));
    }

    return unauthorized;
}

#[utoipa::path(
    get,
    path = "/bearer",
    tag = "Auth",
    responses(
        (status = 200, description = "Successful authentication", content_type = "application/json"),
        (status = 401, description = "Unsuccessful authentication", content_type = "application/json")
    )
)]
/// Prompts the user for authorization using bearer authentication.
pub fn bearer(req: &Request) -> Result<Response, Error> {
    let unauthorized = Ok(Response::from_status(StatusCode::UNAUTHORIZED)
        .with_content_type(mime::APPLICATION_JSON));

    match req.get_header("authorization") {
        Some(auth) => {
            let token = auth.to_str().unwrap_or_default().strip_prefix("Bearer ");
            if token.is_none() {
                return unauthorized;
            }

            let resp = json!({
                "authenticated": true,
                "token": token,
            });

            Ok(Response::from_status(StatusCode::OK)
                .with_content_type(mime::APPLICATION_JSON)
                .with_body(to_string_pretty(&resp).unwrap_or_default()))
        },
        None => unauthorized,
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use serde_json::Value;

    #[test]
    fn test_basic_auth_success() {
        let req = &Request::from_client()
            .with_path("/basic-auth/foo/bar")
            .with_header("authorization", "Basic Zm9vOmJhcg=="); // echo -n foo:bar | base64
        let resp = basic_auth(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
    }

    #[test]
    fn test_basic_auth_incorrect_format() {
        let req = &Request::from_client()
            .with_path("/basic-auth/foo/bar")
            .with_header("authorization", "Basic foo");
        let resp = basic_auth(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::UNAUTHORIZED);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
    }

    #[test]
    fn test_basic_auth_no_authorization() {
        let req = &Request::from_client()
            .with_path("/basic-auth/foo/bar");
        let resp = basic_auth(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::UNAUTHORIZED);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
        assert_eq!(resp.get_header_str("www-authenticate"), Some("Basic Realm=\"Fake Realm\""));
    }

    #[test]
    fn test_basic_auth_wrong_password() {
        let req = &Request::from_client()
            .with_header("authorization", "Bearer Zm9vOmZvbwo=")
            .with_path("/basic-auth/foo/bar");
        let resp = basic_auth(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::UNAUTHORIZED);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
    }

    #[test]
    fn test_bearer_success() {
        let req = &Request::from_client()
            .with_path("/bearer")
            .with_header("authorization", "Bearer foo");
        let resp = bearer(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));

        let body = resp.into_body_str();
        let v: Value = serde_json::from_str(body.as_str()).unwrap();
        assert_eq!(v["authenticated"], true);
        assert_eq!(v["token"], "foo");
    }

    #[test]
    fn test_bearer_failure() {
        let req = &Request::from_client()
            .with_path("/bearer"); // No authorization header
        let resp = bearer(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::UNAUTHORIZED);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
    }
}

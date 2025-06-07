use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};

#[utoipa::path(
    get,
    path = "/cookies",
    tag = "Cookies",
    responses(
        (status = 200, description = "Returns all cookies.", content_type = "application/json"),
    )
)]
/// Returns cookie data.
pub fn get_cookies(req: &Request) -> Result<Response, Error> {
    let cookies: Vec<(String, String)> = req.get_header_str("cookie")
        .map(|cookie_str| {
            cookie_str.split(';')
                .filter_map(|cookie| {
                    let mut parts = cookie.trim().splitn(2, '=');
                    let name = parts.next()?.trim().to_string();
                    let value = parts.next()?.trim().to_string();
                    Some((name, value))
                })
                .collect()
        })
        .unwrap_or_default();

    let cookies_map: serde_json::Map<String, serde_json::Value> = cookies
        .into_iter()
        .map(|(name, value)| (name, json!(value)))
        .collect();

    let resp = json!({
        "cookies": cookies_map,
    });

    Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_body(to_string_pretty(&resp).unwrap_or_default()))
}

#[utoipa::path(
    get,
    path = "/cookies/set/{name}/{value}",
    tag = "Cookies",
    params(
        ("name" = String, Path, description = "Name of the cookie to set"),
        ("value" = String, Path, description = "Value to set for the cookie")
    ),
    responses(
        (status = 302, description = "Sets a cookie and redirects to /cookies", content_type = "application/json"),
    )
)]
/// Sets a cookie and redirects to /cookies.
pub fn set_cookie(req: &Request) -> Result<Response, Error> {
    use regex_lite::Regex;

    let caps = Regex::new(r"/cookies/set/([^/]+)/([^/]+)$")?
        .captures(req.get_path());
    
    if let Some(caps) = caps {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        let value = caps.get(2).map(|m| m.as_str()).unwrap_or("");
        
        return Ok(Response::from_status(StatusCode::FOUND)
            .with_header("Location", "/cookies")
            .with_header("Set-Cookie", format!("{}={}; Path=/", name, value)));
    }

    Ok(Response::from_status(StatusCode::BAD_REQUEST)
        .with_content_type(mime::TEXT_PLAIN)
        .with_body("Invalid cookie parameters"))
}

#[utoipa::path(
    get,
    path = "/cookies/delete/{name}",
    tag = "Cookies",
    params(
        ("name" = String, Path, description = "Name of the cookie to delete")
    ),
    responses(
        (status = 302, description = "Deletes a cookie and redirects to /cookies", content_type = "application/json"),
    )
)]
/// Deletes a cookie and redirects to /cookies.
pub fn delete_cookie(req: &Request) -> Result<Response, Error> {
    use regex_lite::Regex;

    let caps = Regex::new(r"/cookies/delete/([^/]+)$")?
        .captures(req.get_path());
    
    if let Some(caps) = caps {
        let name = caps.get(1).map(|m| m.as_str()).unwrap_or("");
        
        return Ok(Response::from_status(StatusCode::FOUND)
            .with_header("Location", "/cookies")
            .with_header("Set-Cookie", format!("{}=; Path=/; Expires=Thu, 01 Jan 1970 00:00:00 GMT", name)));
    }

    Ok(Response::from_status(StatusCode::BAD_REQUEST)
        .with_content_type(mime::TEXT_PLAIN)
        .with_body("Invalid cookie name"))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cookies() {
        let req = &Request::from_client()
            .with_header("cookie", "foo=bar; baz=qux");
        let resp = get_cookies(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::APPLICATION_JSON));
        let json_str = resp.into_body_str();
        let json: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        assert_eq!(json["cookies"]["foo"], "bar");
        assert_eq!(json["cookies"]["baz"], "qux");
    }

    #[test]
    fn test_set_cookie() {
        let req = &Request::from_client()
            .with_path("/cookies/set/foo/bar");
        let resp = set_cookie(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_header_str("Location"), Some("/cookies"));
        assert_eq!(resp.get_header_str("Set-Cookie"), Some("foo=bar; Path=/"));
    }

    #[test]
    fn test_delete_cookie() {
        let req = &Request::from_client()
            .with_path("/cookies/delete/foo");
        let resp = delete_cookie(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_header_str("Location"), Some("/cookies"));
        assert!(resp.get_header_str("Set-Cookie").unwrap().contains("foo="));
        assert!(resp.get_header_str("Set-Cookie").unwrap().contains("Expires="));
    }
}

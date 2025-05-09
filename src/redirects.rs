use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use regex::Regex;

#[utoipa::path(
    get,
    path = "/absolute-redirect/{n}",
    tag = "Redirects",
    params(
        ("n" = u16, Path, description = "Number of times to redirect"),
    ),
    responses(
        (status = 302, description = "A redirection.", content_type = "text/html"),
    )
)]
pub fn absolute_redirect(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/absolute-redirect/(\d{1})$")?
        .captures(req.get_path());

    if let Some(caps) = caps {
        let n = caps.get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap_or(404));
        let url = req.get_url();
        let base_url = format!("{}://{}/", url.scheme(), url.host_str().unwrap_or_default());

        let redirect_to = {
            if n > 1 {
                format!("{}/absolute-redirect/{}", base_url, n-1)
            } else {
                format!("{}/get", base_url)
            }
        };

        // Return a 302 redirect with an absolute url
        return Ok(Response::from_status(StatusCode::FOUND)
            .with_header("location", redirect_to)
            .with_content_type(mime::TEXT_HTML_UTF_8));
    }

    // If the regex doesn't match, return a 404
    Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8))
}

#[utoipa::path(
    get,
    path = "/relative-redirect/{n}",
    tag = "Redirects",
    params(
        ("n" = u16, Path, description = "Number of times to redirect"),
    ),
    responses(
        (status = 302, description = "A redirection.", content_type = "text/html"),
    )
)]
/// Relatively 302 redirects n times.
pub fn relative_redirect(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/(?:relative-)?redirect/(\d{1})$")?
        .captures(req.get_path());
    if let Some(caps) = caps {
        let n = caps.get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap_or(404));
        let redirect_to = {
            if n > 1 {
                format!("/relative-redirect/{}", n-1)
            } else {
                String::from("/get")
            }
        };
        return Ok(Response::from_status(StatusCode::FOUND)
            .with_header("location", redirect_to)
            .with_content_type(mime::TEXT_HTML_UTF_8));
    }
    Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8))
}

#[utoipa::path(
    get,
    path = "/redirect/{n}",
    tag = "Redirects",
    params(
        ("n" = u16, Path, description = "Number of times to redirect"),
    ),
    responses(
        (status = 302, description = "A redirection.", content_type = "text/html"),
    )
)]
/// 302 redirects n times.
pub fn redirect(req: &Request) -> Result<Response, Error> {
    relative_redirect(req)
}

#[cfg(test)]
mod test {
    use super::*;
    use fastly::http::HeaderValue;

    #[test]
    fn test_absolute_redirect_success() {
        let req = &Request::from_client()
            .with_path("/absolute-redirect/4");
        let resp = absolute_redirect(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
        assert_eq!(resp.get_header("location"), Some(&HeaderValue::from_static("http://example.com/absolute-redirect/3")));
    }

    #[test]
    fn test_absolute_redirect_too_many_redirects() {
        let req = &Request::from_client()
            .with_path("/absolute-redirect/41");
        let resp = absolute_redirect(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }

    #[test]
    fn test_relative_redirect_success() {
        let req = &Request::from_client()
            .with_path("/relative-redirect/3");
        let resp = relative_redirect(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
        assert_eq!(resp.get_header("location"), Some(&HeaderValue::from_static("/relative-redirect/2")));
    }

    #[test]
    fn test_relative_redirect_too_many_redirects() {
        let req = &Request::from_client()
            .with_path("/relative-redirect/15");
        let resp = relative_redirect(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }

    #[test]
    fn test_redirect() {
        let req = &Request::from_client()
            .with_path("/redirect/5");
        let resp = redirect(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
        assert_eq!(resp.get_header("location"), Some(&HeaderValue::from_static("/relative-redirect/4")));
    }
}

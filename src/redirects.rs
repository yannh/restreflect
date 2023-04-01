use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use regex::Regex;

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
    let caps = Regex::new(r"/relative-redirect/(\d{1})$")?
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

#[test]
fn test_relative_redirect() {
    let req = &fastly::Request::from_client()
        .with_path("/relative-redirect/3");
    let resp = redirect(req);
    assert!(resp.is_ok());
    let resp = resp.unwrap();
    assert_eq!(resp.get_status(), StatusCode::FOUND);
    assert_eq!(resp.get_content_type().unwrap(), mime::TEXT_HTML_UTF_8);
    let location = resp.get_header("location");
    assert!(location.is_some());
    assert_eq!(location.unwrap(), "/relative-redirect/2");
}

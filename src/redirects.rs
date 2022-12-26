use fastly::http::{StatusCode};
use fastly::{Error, mime, Request, Response};
use serde_json::{json, to_string_pretty};
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
    let caps = Regex::new(r"/relative-redirect/(\d{1})$").unwrap()
        .captures(req.get_path());
    if caps.is_some() {
        let n = caps.unwrap().get(1).map_or(404, |m| m.as_str().parse::<u16>().unwrap_or(404));
        let mut redirect_to = String::from("");
        if n > 1 {
            redirect_to = format!("/relative-redirect/{}", n-1)
        } else {
            redirect_to = String::from("/get")
        }
        return Ok(Response::from_status(StatusCode::FOUND)
            .with_header("location", redirect_to)
            .with_content_type(mime::TEXT_HTML_UTF_8));
    }
    return Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML))
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
    return relative_redirect(req)
}
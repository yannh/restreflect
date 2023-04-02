use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use regex::Regex;
use rand::seq::SliceRandom;

fn rr_http_statuses(req: &Request) -> Result<Response, Error> {
    let caps = Regex::new(r"/status/((\d{3},?)+)$")?
        .captures(req.get_path());
    if let Some(caps) = caps {
        let statuses:Vec<&str> = caps.get(1).map_or("404", |m| m.as_str()).split(",").collect();
        let status = statuses.choose(&mut rand::thread_rng()).unwrap().parse::<u16>().unwrap_or(404);
        return Ok(Response::from_status(StatusCode::from_u16(status).unwrap_or(StatusCode::NOT_FOUND))
            .with_content_type(mime::TEXT_HTML_UTF_8));
    }

    Ok(Response::from_status(StatusCode::NOT_FOUND)
        .with_content_type(mime::TEXT_HTML_UTF_8))
}

#[utoipa::path(
    get,
    path = "/status/{codes}",
    operation_id = "status_get",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn get(req: &Request) -> Result<Response, Error> {
    rr_http_statuses(req)
}

#[utoipa::path(
    post,
    path = "/status/{codes}",
    operation_id = "status_post",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn post(req: &mut Request) -> Result<Response, Error> {
    rr_http_statuses(req)
}

#[utoipa::path(
    put,
    path = "/status/{codes}",
    operation_id = "status_put",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn put(req: &mut Request) -> Result<Response, Error> {
    rr_http_statuses(req)
}

#[utoipa::path(
    patch,
    path = "/status/{codes}",
    operation_id = "status_patch",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn patch(req: &mut Request) -> Result<Response, Error> {
    rr_http_statuses(req)
}

#[utoipa::path(
    delete,
    path = "/status/{codes}",
    operation_id = "status_delete",
    tag = "Status codes",
    responses(
        (status = 100, description = "Informational Response"),
        (status = 200, description = "Success"),
        (status = 300, description = "Redirection"),
        (status = 400, description = "Client Errors"),
        (status = 500, description = "Server Errors"),
    ),
    params(
        ("codes" = u16, Path, description = "Return status code or random status code if more than one are given"),
    )
)]
/// Return status code or random status code if more than one is given
pub fn delete(req: &Request) -> Result<Response, Error> {
    rr_http_statuses(req)
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_get_200() {
        let req = &Request::from_client()
            .with_path("/status/200");
        let resp = get(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }

    #[test]
    fn test_get_500() {
        let req = &Request::from_client()
            .with_path("/status/500");
        let resp = get(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }

    #[test]
    fn test_post_302() {
        let mut req = Request::from_client()
            .with_path("/status/302");
        let resp = post(&mut req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }

    #[test]
    fn test_post_non_existing() {
        let mut req = Request::from_client()
            .with_path("/status/9999");
        let resp = post(&mut req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();
        assert_eq!(resp.get_status(), StatusCode::NOT_FOUND);
        assert_eq!(resp.get_content_type(), Some(mime::TEXT_HTML_UTF_8));
    }
}

use fastly::http::StatusCode;
use fastly::{Error, mime, Request, Response};
use crate::utils::req_to_json;
use deflate::{deflate_bytes, deflate_bytes_gzip};

#[utoipa::path(
    get,
    path = "/brotli",
    tag = "Response formats",
    responses(
    (status = 200, description = "Brotli-encoded data.", content_type = "application/json")
    )
)]
/// Returns Brotli-encoded data.
pub fn brotli(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let mut enc = vec!();
    let params = brotli::enc::BrotliEncoderParams::default();
    match brotli::BrotliCompress(&mut res.as_bytes(), &mut enc, &params) {
        Ok(_) => {},
        Err(e) => panic!("Error {:?}", e),
    }
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "br")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/deflate",
    tag = "Response formats",
    responses(
        (status = 200, description = "Deflate-encoded data.", content_type = "application/json")
    )
)]
/// Returns Deflate-encoded data.
pub fn deflate(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let enc = deflate_bytes(res.as_bytes());
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "deflate")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/gzip",
    tag = "Response formats",
    responses(
        (status = 200, description = "GZip-encoded data.", content_type = "application/json")
    )
)]
/// Returns GZip-encoded data.
pub fn gzip(req: &Request) -> Result<Response, Error> {
    let res = req_to_json(req);
    let enc = deflate_bytes_gzip(res.as_bytes());
    return Ok(Response::from_status(StatusCode::OK)
        .with_content_type(mime::APPLICATION_JSON)
        .with_header("content-encoding", "gzip")
        .with_body(enc))
}

#[utoipa::path(
    get,
    path = "/html",
    tag = "Response formats",
    responses(
        (status = 200, description = "An HTML page.", content_type = "text/html")
    )
)]
/// Returns a simple HTML document.
pub fn html(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("html.html", mime::TEXT_HTML);
}

#[utoipa::path(
    get,
    path = "/json",
    tag = "Response formats",
    responses(
        (status = 200, description = "A JSON document.", content_type = "application/json")
    )
)]
/// Returns a simple JSON document.
pub fn json(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("json.json", mime::APPLICATION_JSON);
}

#[utoipa::path(
    get,
    path = "/robots.txt",
    tag = "Response formats",
    responses(
        (status = 200, description = "Robots file", content_type = "text/plain")
    )
)]
/// Returns some robots.txt rules.
pub fn robots_txt(_: &Request) -> Result<Response, Error> {
    return crate::assets::serve("robots.txt", mime::TEXT_PLAIN);
}

#[utoipa::path(
    get,
    path = "/xml",
    tag = "Response formats",
    responses(
        (status = 200, description = "A XML document.", content_type = "application/xml")
    )
)]
/// Returns a simple XML document.
pub fn xml(_: &Request) -> Result<Response, Error> {
    let mime_xml: mime::Mime = "application/xml".parse().unwrap_or(mime::APPLICATION_OCTET_STREAM);
    crate::assets::serve("xml.xml", mime_xml)
}

#[utoipa::path(
    get,
    path = "/deny",
    tag = "Response formats",
    responses(
        (status = 200, description = "Denied message", body=str, content_type = "text/plain")
    )
)]
/// Returns page denied by robots.txt rules.
pub fn deny(_: &Request) -> Result<Response, Error> {
    crate::assets::serve("deny.txt", mime::TEXT_PLAIN)
}

#[utoipa::path(
    get,
    path = "/encoding/utf8",
    tag = "Response formats",
    responses(
        (status = 200, description = "Encoded UTF-8 content.", body=str, content_type = "text/plain")
    )
)]
/// Returns a UTF-8 encoded body.
pub fn encoding_utf8(_: &Request) -> Result<Response, Error> {
    crate::assets::serve("utf8.txt", mime::TEXT_PLAIN)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deflate_with_valid_request() {
        let req = &Request::from_client()
            .with_path("/deflate?foo=bar");

        let resp = deflate(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();

        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_header("content-type").unwrap(), "application/json");
        assert_eq!(resp.get_header("content-encoding").unwrap(), "deflate");
    }

    #[test]
    fn test_json() {
        let req = &Request::from_client()
            .with_path("/json");

        let resp = json(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();

        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_header("content-type").unwrap(), "application/json");
        assert_eq!(resp.into_body_str(), "{\n  \"slideshow\": {\n    \"author\": \"Yours Truly\", \n    \"date\": \"date of publication\", \n    \"slides\": [\n      {\n        \"title\": \"Wake up to WonderWidgets!\", \n        \"type\": \"all\"\n      }, \n      {\n        \"items\": [\n          \"Why <em>WonderWidgets</em> are great\", \n          \"Who <em>buys</em> WonderWidgets\"\n        ], \n        \"title\": \"Overview\", \n        \"type\": \"all\"\n      }\n    ], \n    \"title\": \"Sample Slide Show\"\n  }\n}\n");
    }

    #[test]
    fn test_xml() {
        let req = &Request::from_client()
            .with_header("accept", "*/*")
            .with_path("/xml");

        let resp = xml(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();

        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_header("content-type").unwrap(), "application/xml");
        assert_eq!(resp.into_body_str(), "<?xml version='1.0' encoding='us-ascii'?>\n\n<!--  A SAMPLE set of slides  -->\n\n<slideshow \n    title=\"Sample Slide Show\"\n    date=\"Date of publication\"\n    author=\"Yours Truly\"\n    >\n\n    <!-- TITLE SLIDE -->\n    <slide type=\"all\">\n      <title>Wake up to WonderWidgets!</title>\n    </slide>\n\n    <!-- OVERVIEW -->\n    <slide type=\"all\">\n        <title>Overview</title>\n        <item>Why <em>WonderWidgets</em> are great</item>\n        <item/>\n        <item>Who <em>buys</em> WonderWidgets</item>\n    </slide>\n\n</slideshow>");
    }

    #[test]
    fn test_robots_txt() {
        let req = &Request::from_client()
            .with_header("accept", "*/*")
            .with_path("/robots");

        let resp = robots_txt(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();

        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_header("content-type").unwrap(), "text/plain");
        assert_eq!(resp.into_body_str(), "User-agent: *\nDisallow: /deny\n");
    }

    #[test]
    fn test_deny() {
        let req = &Request::from_client()
            .with_header("accept", "text/plain")
            .with_path("/deny");

        let resp = deny(req);
        assert!(resp.is_ok());
        let resp = resp.unwrap();

        assert_eq!(resp.get_status(), StatusCode::OK);
        assert_eq!(resp.get_header("content-type").unwrap(), "text/plain");
        assert_eq!(resp.into_body_str(), "\n          .-''''''-.\n        .' _      _ '.\n       /   O      O   \\\n      :                :\n      |                |\n      :       __       :\n       \\  .-\"`  `\"-.  /\n        '.          .'\n          '-......-'\n     YOU SHOULDN'T BE HERE\n");
    }
}

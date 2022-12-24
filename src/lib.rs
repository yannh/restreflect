use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use std::collections::HashMap;
use serde_json::{json, to_string_pretty};

pub fn req_to_json(req: Request) -> String {
    let headers: HashMap<&str, &str>= req.get_headers()
        .map(|m| (m.0.as_str(), m.1.to_str().unwrap_or("")))
        .collect();

    let resp = json!({
            "headers": headers,
            "origin": req.get_client_ip_addr(),
            "url": req.get_url_str()
        });

    return to_string_pretty(&resp).unwrap();
}

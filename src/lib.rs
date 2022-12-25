use fastly::http::{Method, StatusCode};
use fastly::{Error, mime, Request, Response};
use std::collections::HashMap;
use serde_json::{json, to_string_pretty};


pub fn req_headers(req: &Request) -> HashMap<&str, &str> {
    return req.get_headers()
        .map(|m| (m.0.as_str(), m.1.to_str().unwrap_or("")))
        .collect();
}

pub fn req_to_json(req: &Request) -> String {
    let arg_pairs: Vec<(String, String)> = req.get_query().unwrap();
    let args: HashMap<&str, &str> = arg_pairs.iter().map(|m| (m.0.as_str(), m.1.as_str()))
        .collect();

    let resp = json!({
            "args": args,
            "headers": req_headers(req),
            "origin": req.get_client_ip_addr(),
            "url": req.get_url_str()
        });

    return to_string_pretty(&resp).unwrap();
}

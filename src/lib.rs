use fastly::http::Method;
use fastly::Request;
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

pub fn req_with_body_to_json(req: &mut Request) -> String {
    let arg_pairs: Vec<(String, String)> = req.get_query().unwrap();
    let args: HashMap<&str, &str> = arg_pairs.iter().map(|m| (m.0.as_str(), m.1.as_str()))
        .collect();

    let resp = match *req.get_method() {
        Method::POST => {
            let f: Vec<(String, String)>;
            let mut fo: HashMap<&str, &str> = HashMap::new();

            if req.get_header("content-type").unwrap() == "application/x-www-form-urlencoded" {
                f = req.take_body_form::<Vec<(String, String)>>().unwrap();
                fo = f
                    .iter()
                    .map(|m| (m.0.as_str(), m.1.as_str()))
                    .collect();
            }

            json!({
            "args": args,
            "form": fo,
            "headers": req_headers(req),
            "origin": req.get_client_ip_addr(),
            "url": req.get_url_str()
            })
        },
        _ => json!({
            "args": args,
            "headers": req_headers(req),
            "origin": req.get_client_ip_addr(),
            "url": req.get_url_str()
        }),
    };

    return to_string_pretty(&resp).unwrap();
}

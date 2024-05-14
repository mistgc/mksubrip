use crate::prelude::*;

use reqwest::blocking;
use reqwest::IntoUrl;

use std::collections::HashMap;

const BASE_URL: &str = "http://localhost:5000";

pub fn get<U: IntoUrl>(url: U) -> reqwest::Result<blocking::Response> {
    let client = blocking::Client::new();

    client.get(url).send()
}

pub fn post_with_multipart<U: Into<String>>(
    url: U,
    multipart: blocking::multipart::Form,
) -> reqwest::Result<blocking::Response> {
    let full_url = format!("{BASE_URL}{}", url.into().as_str());
    debug!("Full URL = {}", full_url);
    let client = blocking::Client::new();
    let request_builder = client
        .post(full_url)
        .multipart(multipart)
        .timeout(std::time::Duration::new(0xffffff, 0));

    request_builder.send()
}

pub fn post_with_form<U: Into<String>, K, V>(
    _url: U,
    _map: HashMap<K, V>,
) -> reqwest::Result<blocking::Response> {
    todo!()
}

pub fn post_with_json<U: Into<String>, K, V>(
    _url: U,
    _map: HashMap<K, V>,
) -> reqwest::Result<blocking::Response> {
    todo!()
}

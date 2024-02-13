use actix_web::{
    web::{Bytes, Query},
    HttpRequest,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

pub fn get_req_headers<'a>(req: &'a HttpRequest) -> Vec<(&'a str, &'a str)> {
    req.headers()
        .iter()
        .map(|(name, value)| (name.as_str(), value.to_str().unwrap_or("")))
        .collect()
}

pub fn get_body_data<'a>(bytes: &'a Bytes) -> Cow<'a, str> {
    String::from_utf8_lossy(&bytes)
}

#[derive(Debug, Deserialize, Serialize)]
pub struct QParams {
    pub id: Option<u8>,
    pub message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData<'a> {
    pub message: &'a str,
    pub status_code: String,
    pub body_data: Cow<'a, str>,
    pub queries: QParams,
    pub headers: Vec<(&'a str, &'a str)>,
}

impl<'a> ResponseData<'a> {
    pub fn new() -> Self {
        ResponseData {
            message: "Success",
            status_code: String::from("200"),
            body_data: String::new().into(),
            queries: QParams {
                id: None,
                message: None,
            },
            headers: Vec::new(),
        }
    }

    pub fn message(mut self, message: &'a str) -> Self {
        self.message = message;
        self
    }

    pub fn status_code(mut self, status_code: String) -> Self {
        self.status_code = status_code;
        self
    }

    pub fn body(mut self, body: Cow<'a, str>) -> Self {
        self.body_data = body;
        self
    }

    pub fn queries(mut self, queries: Query<QParams>) -> Self {
        self.queries = queries.into_inner();
        self
    }

    pub fn headers(mut self, headers: Vec<(&'a str, &'a str)>) -> Self {
        self.headers = headers;
        self
    }
}

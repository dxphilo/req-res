use actix_web::{web::Bytes, HttpRequest};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub fn get_req_headers(req: &HttpRequest) -> HashMap<String, String> {
    req.headers()
        .iter()
        .map(|(name, value)| {
            (
                name.as_str().to_string(),
                value.to_str().unwrap_or("").to_string(),
            )
        })
        .collect()
}

pub fn get_body_data(bytes: &Bytes) -> Cow<'_, str> {
    String::from_utf8_lossy(bytes)
}

pub fn get_req_queries(req: &HttpRequest)->HashMap<String, String>{
    let req_queries = req
        .uri()
        .query()
        .map(|q| {
            q.split('&')
                .filter_map(|param| {
                    let mut parts = param.split('=');
                    let key = parts.next()?;
                    let value = parts.next()?;
                    // Check if the key and value match the desired criteria
                    Some((key.to_owned(), value.to_owned()))
                })
                .collect()
        })
        .unwrap_or_else(|| HashMap::new());
    req_queries
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Method {
    POST,
    GET,
    PUT,
    DELETE,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseData<'a> {
    pub message: &'a str,
    pub status_code: String,
    pub method: Method,
    pub path: String,
    pub body_data: Cow<'a, str>,
    pub queries: HashMap<String, String>,
    pub headers: HashMap<String, String>,
}

impl<'a> ResponseData<'a> {
    pub fn new() -> Self {
        ResponseData {
            message: "Success",
            status_code: String::from("200"),
            method: Method::GET,
            body_data: String::new().into(),
            queries: HashMap::new(),
            headers: HashMap::new(),
            path: String::new(),
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

    pub fn method(mut self, req_method: Method) -> Self {
        self.method = req_method;
        self
    }
    pub fn path(mut self, req_path: String) -> Self {
        self.path = req_path;
        self
    }

    pub fn body(mut self, body: Cow<'a, str>) -> Self {
        self.body_data = body;
        self
    }

    pub fn queries(mut self, queries: HashMap<String, String>) -> Self {
        self.queries = queries;
        self
    }

    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers = headers;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::header;
    use actix_web::test::TestRequest;

    #[test]
    fn test_get_req_headers() {
        // Create a test HttpRequest with some headers
        let req = TestRequest::default()
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .insert_header((header::USER_AGENT, "Actix-Web"))
            .to_http_request();

        // Call the function to get request headers
        let headers = get_req_headers(&req);

        // Assert the expected headers are present
        assert_eq!(headers.len(), 2);
        assert_eq!(
            headers.get("content-type"),
            Some(&"application/json".to_string())
        );
        assert_eq!(headers.get("user-agent"), Some(&"Actix-Web".to_string()));
    }

    #[test]
    fn test_get_body_data() {
        // Create a test Bytes object
        let bytes = Bytes::from("Test body data");

        // Call the function to get body data
        let body_data = get_body_data(&bytes);

        // Assert the expected body data
        assert_eq!(body_data, "Test body data");
    }

    #[test]
    fn test_response_data_builder() {
        // Create a sample Query<QParams>
        let query_params = vec![
            ("content-type".to_string(), "application/json".to_string()),
            // Add more test queries as needed
        ];
        let mut query_map = HashMap::new();
        for (key, value) in query_params {
            query_map.insert(key, value);
        }
        let headers_vec = vec![
            ("content-type".to_string(), "application/json".to_string()),
            // Add more test headers as needed
        ];

        let mut headers_map = HashMap::new();
        for (key, value) in headers_vec {
            headers_map.insert(key, value);
        }

        let response_data = ResponseData::new()
            .message("Custom message")
            .status_code("404".to_string())
            .method(Method::POST)
            .body("Custom body".into())
            .queries(query_map)
            .headers(headers_map);

        assert_eq!(response_data.message, "Custom message");
        assert_eq!(response_data.status_code, "404");
        assert_eq!(response_data.body_data, "Custom body");
        assert_eq!(response_data.headers.len(), 1);
        assert_eq!(
            response_data.headers.get("content-type"),
            Some(&"application/json".to_string())
        );
    }
}

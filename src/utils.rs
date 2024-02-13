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
        assert_eq!(headers[0], ("content-type", "application/json"));
        assert_eq!(headers[1], ("user-agent", "Actix-Web"));
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
        let query_params = Query(QParams {
            id: Some(123),
            message: Some("Test message".to_string()),
        });

        // Call the ResponseData builder methods
        let response_data = ResponseData::new()
            .message("Custom message")
            .status_code("404".to_string())
            .body("Custom body".into())
            .queries(query_params)
            .headers(vec![("content-type", "application/json")]);

        // Assert the attributes of the ResponseData
        assert_eq!(response_data.message, "Custom message");
        assert_eq!(response_data.status_code, "404");
        assert_eq!(response_data.body_data, "Custom body");
        assert_eq!(response_data.queries.id, Some(123));
        assert_eq!(
            response_data.queries.message,
            Some("Test message".to_string())
        );
        assert_eq!(response_data.headers.len(), 1);
        assert_eq!(
            response_data.headers[0],
            ("content-type", "application/json")
        );
    }
}

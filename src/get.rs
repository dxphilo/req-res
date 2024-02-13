use crate::utils::{get_body_data, get_req_headers, QParams, ResponseData};
use actix_web::{
    get,
    web::{Bytes, Query},
    HttpRequest, HttpResponse,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct SuccessResponse {
    name: String,
    about: String,
    created_by: String,
    message: String,
    status_code: u16,
}

impl SuccessResponse {
    fn new() -> Self {
        SuccessResponse {
            name: "Welcome to Gentoo".to_string(),
            about: String::from("Lightweight Rest API Mock Client"),
            created_by: String::from("John Philip"),
            message: "Success".to_string(),
            status_code: 200,
        }
    }
}

#[get("/")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse::new())
}

#[get("/get")]
pub async fn get_responder(req: HttpRequest, bytes: Bytes, query: Query<QParams>) -> HttpResponse {
    let body_data = get_body_data(&bytes);

    let headers = get_req_headers(&req);

    let response_data = ResponseData::new()
        .message("Request successfull")
        .status_code("200".to_string())
        .body(body_data)
        .queries(query)
        .headers(headers);

    HttpResponse::Ok().json(response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[test]
    async fn test_get_responder() {
        let mut app = test::init_service(App::new().service(get_responder)).await;

        let req = test::TestRequest::get()
            .uri("/get?id=123&message=test_message")
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);

        // Read the response body
        let resp_body = test::read_body(resp).await;

        let response_data: ResponseData =
            serde_json::from_slice(&resp_body).expect("Failed to parse response body");

        let query_params = QParams {
            id: Some(123),
            message: Some("test_message".to_string()),
        };

        assert_eq!(query_params.id, Some(123));
        assert_eq!(query_params.message, Some("test_message".to_string()));
        assert_eq!(response_data.message, "Request successfull");
        assert_eq!(response_data.status_code, "200");
    }

    #[test]
    async fn test_health_check() {
        let mut app = test::init_service(App::new().service(health_check)).await;

        let req = test::TestRequest::get().uri("/").to_request();

        let resp: SuccessResponse = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.status_code, 200);

        assert_eq!(resp.message, "Success");
        assert_eq!(resp.name, "Welcome to Gentoo");
        assert_eq!(resp.about, "Lightweight Rest API Mock Client");
        assert_eq!(resp.created_by, "John Philip");
    }
}

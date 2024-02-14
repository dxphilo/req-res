use crate::utils::{get_body_data, get_req_headers, QParams, ResponseData};
use actix_web::{
    post,
    web::{Bytes, Query},
    HttpRequest, HttpResponse,
};

#[post("/post")]
pub async fn post_responder(req: HttpRequest, bytes: Bytes, query: Query<QParams>) -> HttpResponse {
    let body_data = get_body_data(&bytes);

    let headers = get_req_headers(&req);

    let response_data = ResponseData::new()
        .message("Request successfull")
        .status_code("201".to_string())
        .method(crate::utils::Method::POST)
        .body(body_data)
        .queries(query)
        .headers(headers);

    HttpResponse::Created().json(response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http::StatusCode, test, App};

    #[test]
    async fn test_post_responder() {
        let mut app = test::init_service(App::new().service(post_responder)).await;

        let request_body = "Test request body";

        let req = test::TestRequest::post()
            .uri("/post?id=123&message=test_message")
            .set_payload(request_body)
            .to_request();

        let resp = test::call_service(&mut app, req).await;

        assert_eq!(resp.status(), StatusCode::CREATED);

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
        assert_eq!(response_data.status_code, "201");
        assert_eq!(response_data.body_data, "Test request body");
    }
}

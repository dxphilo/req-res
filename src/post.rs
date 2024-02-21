use crate::utils::{get_body_data, get_req_headers, get_req_queries, ResponseData};
use actix_web::{
    post,
    web::Bytes,
    HttpRequest, HttpResponse,
};

#[post("/post")]
pub async fn post_responder(req: HttpRequest, bytes: Bytes) -> HttpResponse {
    let path = req.uri().path().to_string();
    let headers = get_req_headers(&req);
    let body_str =  get_body_data(&bytes);

    let query_params = get_req_queries(&req);

    let response_data = ResponseData::new()
        .message("Request successfull")
        .status_code("201".to_string())
        .method(crate::utils::Method::POST)
        .path(path)
        .body(body_str)
        .queries(query_params)
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

        assert_eq!(response_data.message, "Request successfull");
        assert_eq!(response_data.status_code, "201");
        assert_eq!(response_data.body_data, "Test request body");
    }
}

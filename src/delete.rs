use crate::utils::{get_body_data, get_req_headers, get_req_queries, ResponseData};
use actix_web::{ delete, web::Bytes, HttpRequest, HttpResponse
};

#[delete("/delete")]
pub async fn delete_responder(
    req: HttpRequest,
    body: Bytes,
) -> HttpResponse {
    let path = req.uri().path().to_string();
    let headers = get_req_headers(&req);
    let body_str =  get_body_data(&body);

    let query_params = get_req_queries(&req);

    let response_data = ResponseData::new()
        .message("Request successfull")
        .status_code("200".to_string())
        .method(crate::utils::Method::DELETE)
        .path(path)
        .body(body_str)
        .queries(query_params)
        .headers(headers);

    HttpResponse::Ok().json(response_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{self},
        test, App,
    };
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Person {
        id: String,
        name: String,
    }

    #[test]
    async fn test_delete_responder_missing_headers() {
        // Initialize the service with the delete_responder
        let app = test::init_service(App::new().service(delete_responder)).await;

        // Mock DELETE request to /delete endpoint
        let req = test::TestRequest::delete().uri("/delete").to_request();

        // Send the request to the service
        let resp = test::call_service(&app, req).await;
        println!("response : {:?}", resp);

        // Assert that the response status is success (200 OK)
        assert_eq!(resp.status(), http::StatusCode::OK);

        // Assert other response properties as needed
    }

    #[test]
    async fn test_delete_responder_with_values_and_body_data() {
        // Initialize service (replace with your implementation)
        let mut app = test::init_service(App::new().service(delete_responder)).await;

        // Prepare payload (replace with your actual data)
        let payload = r#"{"id":"12345","name":"User name"}"#.as_bytes();

        // Mock DELETE request (replace with your specific endpoint and parameters)
        let req = test::TestRequest::delete()
            .insert_header(("Content-Type", "application/json"))
            .uri("/delete?id=123&message=test_message")
            .set_payload(payload)
            .to_request();

        #[derive(Debug, serde::Serialize, serde::Deserialize)]
        struct ResponseBody {
            message: String,
            status_code: String,
            body_data: serde_json::Value,
            queries: serde_json::Value,
            headers: HashMap<String, String>,
        }

        let resp: ResponseBody = test::call_and_read_body_json(&mut app, req).await;
        println!("Response: {:?}", resp);

        assert_eq!(resp.message, "Request successfull".to_string());
        assert_eq!(resp.status_code, "200".to_string());
        assert_eq!(resp.queries["id"], 123);
        assert_eq!(resp.queries["message"], "test_message".to_string());
        // TODO: Add assertions for headers if needed
    }
}

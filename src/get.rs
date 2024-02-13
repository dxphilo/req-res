use actix_web::{get, HttpResponse};
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

#[get("/get")]
pub async fn get_responder() -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse::new())
}

#[get("/")]
pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(SuccessResponse::new())
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[test]
    async fn test_get_responder() {
        let mut app = test::init_service(App::new().service(get_responder)).await;

        let req = test::TestRequest::get().uri("/get").to_request();

        // Send the request to the service
        let resp: SuccessResponse = test::call_and_read_body_json(&mut app, req).await;

        assert_eq!(resp.status_code, 200);
        assert_eq!(resp.message, "Success");
        assert_eq!(resp.name, "Welcome to Gentoo");
        assert_eq!(resp.about, "Lightweight Rest API Mock Client");
        assert_eq!(resp.created_by, "John Philip");
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

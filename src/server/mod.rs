use crate::sender::DynMtbFileSender;
use crate::server::AppResponse::{
    Accepted, BadRequest, Unauthorized, UnprocessableContent, UnsupportedContentType,
};
use crate::{CONFIG, shutdown_signal};
use axum::body::Body;
use axum::http::StatusCode;
use axum::http::header::WWW_AUTHENTICATE;
use axum::response::{IntoResponse, Response};

mod handlers;
pub mod routes;

pub async fn start_server(sender: DynMtbFileSender) -> Result<(), String> {
    match tokio::net::TcpListener::bind(&CONFIG.listen).await {
        Ok(listener) => {
            log::info!("Starting application listening on '{}'", CONFIG.listen);
            if let Err(err) = axum::serve(listener, routes::routes(sender))
                .with_graceful_shutdown(shutdown_signal())
                .await
            {
                return Err(err.to_string());
            }
        }
        Err(err) => return Err(format!("Cannot listening on '{}': {}", CONFIG.listen, err)),
    }

    Ok(())
}

enum AppResponse<'a> {
    Accepted(&'a str),
    BadRequest,
    Unauthorized,
    UnsupportedContentType,
    UnprocessableContent(String),
    InternalServerError,
}

#[allow(clippy::expect_used)]
impl IntoResponse for AppResponse<'_> {
    fn into_response(self) -> Response {
        match self {
            BadRequest => (
                StatusCode::BAD_REQUEST,
                "This application accepts DNPM data model version 2.1 with content type 'application/json'"
            ).into_response(),
            UnsupportedContentType => (
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                "This application accepts DNPM data model version 2.1 with content type 'application/json'"
            ).into_response(),
            UnprocessableContent(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                format!("This application accepts DNPM data model version 2.1 with content type 'application/json'. {err}")
            ).into_response(),
            _ => match self {
                Accepted(request_id) => Response::builder()
                    .status(StatusCode::ACCEPTED)
                    .header("X-Request-Id", request_id),
                Unauthorized => Response::builder()
                    .status(StatusCode::UNAUTHORIZED)
                    .header(WWW_AUTHENTICATE, "Basic realm=\"DNPM Kafka Rest Proxy Realm\""),
                _ => Response::builder().status(StatusCode::INTERNAL_SERVER_ERROR),
            }
                .body(Body::empty()).expect("response built"),
        }
    }
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use axum::http::header::WWW_AUTHENTICATE;
    use axum::response::IntoResponse;
    use uuid::Uuid;

    use crate::server::AppResponse::{Accepted, InternalServerError, Unauthorized};

    #[test]
    fn should_return_success_response() {
        let response = Accepted(&Uuid::new_v4().to_string()).into_response();
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        assert!(response.headers().contains_key("x-request-id"));
    }

    #[test]
    fn should_return_error_response() {
        let response = InternalServerError.into_response();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert!(!response.headers().contains_key("x-request-id"));
    }

    #[test]
    fn should_return_unauthorized_response() {
        let response = Unauthorized.into_response();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert!(response.headers().contains_key(WWW_AUTHENTICATE));
        assert!(!response.headers().contains_key("x-request-id"));
    }
}

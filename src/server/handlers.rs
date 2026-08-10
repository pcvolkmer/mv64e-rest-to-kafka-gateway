use crate::CONFIG;
use crate::sender::{DynMtbFileSender, RequestMethod};
use crate::server::AppResponse::{Accepted, BadRequest, InternalServerError, UnprocessableContent};
use axum::extract::Path;
use axum::extract::rejection::JsonRejection;
use axum::http::HeaderMap;
use axum::response::{IntoResponse, Response};
use axum::{Extension, Json};
use mv64e_mtb_model::models::PatientRecord;

pub async fn handle_delete(
    Path(patient_id): Path<String>,
    Extension(sender): Extension<DynMtbFileSender>,
    headers: HeaderMap,
) -> Response {
    let delete_mtb_file = PatientRecord::new_with_consent_rejected(&patient_id);
    match sender
        .send(
            delete_mtb_file,
            RequestMethod::Delete,
            headers
                .get("x-request-id")
                .map(|v| v.to_str().unwrap_or_default().to_string()),
        )
        .await
    {
        Ok(request_id) => Accepted(&request_id).into_response(),
        _ => InternalServerError.into_response(),
    }
}

pub async fn handle_post(
    Extension(sender): Extension<DynMtbFileSender>,
    headers: HeaderMap,
    payload: Result<Json<PatientRecord>, JsonRejection>,
) -> Response {
    match payload {
        Ok(Json(mtb_file)) => {
            match sender
                .send(
                    mtb_file,
                    RequestMethod::Post,
                    headers
                        .get("x-request-id")
                        .map(|v| v.to_str().unwrap_or_default().to_string()),
                )
                .await
            {
                Ok(request_id) => Accepted(&request_id).into_response(),
                _ => InternalServerError.into_response(),
            }
        }
        // JSON error
        Err(json_rejection) => {
            if CONFIG.send_on_invalid {
                return match sender
                    .send_empty(
                        RequestMethod::Post,
                        headers
                            .get("x-request-id")
                            .map(|v| v.to_str().unwrap_or_default().to_string()),
                    )
                    .await
                {
                    Ok(_) => match json_rejection {
                        JsonRejection::JsonDataError(err) => {
                            UnprocessableContent(err.to_string()).into_response()
                        }
                        _ => BadRequest.into_response(),
                    },
                    _ => InternalServerError.into_response(),
                };
            }

            match json_rejection {
                JsonRejection::JsonDataError(err) => {
                    log::warn!("Invalid JSON data, sending response:\n'{err}'");
                    UnprocessableContent(err.to_string()).into_response()
                }
                _ => BadRequest.into_response(),
            }
        }
    }
}

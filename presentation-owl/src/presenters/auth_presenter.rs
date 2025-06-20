use application_owl::output_ports::AuthOutputPort;
use axum::http::StatusCode;
use axum::{Json, response::IntoResponse, response::Response};

use crate::presenters::auth_messages::*;

pub struct AuthPresenter;

impl Default for AuthPresenter {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthPresenter {
    pub fn new() -> Self {
        AuthPresenter
    }
}

impl AuthOutputPort for AuthPresenter {
    fn login_success(&self) -> Response {
        (StatusCode::OK, Json(LOGIN_SUCCESS)).into_response()
    }
    fn login_failure(&self, message: &str) -> Response {
        (StatusCode::BAD_REQUEST, Json(message)).into_response()
    }
    fn logout_success(&self) -> Response {
        Json(LOGOUT_SUCCESS).into_response()
    }
    fn logout_failure(&self) -> Response {
        (StatusCode::BAD_REQUEST, Json(LOGOUT_FAILURE)).into_response()
    }
    fn auth_check_success(&self, is_authenticated: bool) -> Response {
        let message = if is_authenticated {
            AUTH_CHECK_SUCCESS
        } else {
            AUTH_CHECK_FAILURE
        };
        Json(message).into_response()
    }
    fn auth_check_failure(&self) -> Response {
        (StatusCode::BAD_REQUEST, Json(AUTH_CHECK_ERROR)).into_response()
    }
    fn register_success(&self) -> Response {
        (StatusCode::OK, Json(REGISTER_SUCCESS)).into_response()
    }
    fn register_failure(&self, message: &str) -> Response {
        (
            StatusCode::BAD_REQUEST,
            Json(REGISTER_FAILURE.to_owned() + message),
        )
            .into_response()
    }
}

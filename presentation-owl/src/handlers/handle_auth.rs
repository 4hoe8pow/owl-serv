use anyhow::Result;
use application_owl::interactors::use_case_auth::AuthInteractor;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use infrastructure_owl::repository_impl::employee_repository_impl::EmployeeRepositoryImpl;
use serde_json::Value;
use std::{str::FromStr, sync::Arc};
use worker::Env;

use crate::{
    contollers::auth_controller::AuthController,
    handlers::requests::auth_request::{AuthProcessCode, AuthRequest, RegisterRequest},
    presenters::auth_presenter::AuthPresenter,
};

#[worker::send]
pub async fn handle_auth(
    Path(auth_process_code): Path<String>, // 処理区分
    State(env): State<Arc<Env>>,           // 環境変数
    Json(payload): Json<Value>,            // リクエスト(JSON汎用)
) -> Response {
    let repository = EmployeeRepositoryImpl::new(env.clone());
    let presenter = AuthPresenter::new();
    let interactor = AuthInteractor::new(repository, presenter);
    let controller = AuthController::new(interactor);

    let result: Result<Response> =
        if let Ok(process) = AuthProcessCode::from_str(&auth_process_code) {
            match process {
                AuthProcessCode::Login => {
                    let req: AuthRequest = match serde_json::from_value(payload) {
                        Ok(r) => r,
                        Err(_) => {
                            return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body("Invalid request body".into())
                                .unwrap();
                        }
                    };
                    controller.login(req).await
                }
                AuthProcessCode::Logout => {
                    let req: AuthRequest = match serde_json::from_value(payload) {
                        Ok(r) => r,
                        Err(_) => {
                            return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body("Invalid request body".into())
                                .unwrap();
                        }
                    };
                    controller.logout(req).await
                }
                AuthProcessCode::Check => {
                    let req: AuthRequest = match serde_json::from_value(payload) {
                        Ok(r) => r,
                        Err(_) => {
                            return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body("Invalid request body".into())
                                .unwrap();
                        }
                    };
                    controller.check_auth(req).await
                }
                AuthProcessCode::Register => {
                    let req: RegisterRequest = match serde_json::from_value(payload) {
                        Ok(r) => r,
                        Err(_) => {
                            return Response::builder()
                                .status(StatusCode::BAD_REQUEST)
                                .body("Invalid request body".into())
                                .unwrap();
                        }
                    };
                    controller.register(req).await
                }
            }
        } else {
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body("Invalid auth process code".into())
                .unwrap())
        };

    match result {
        Ok(resp) => resp,
        Err(e) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Internal error: {e}").into())
            .unwrap(),
    }
}

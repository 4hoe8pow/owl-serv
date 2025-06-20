use anyhow::Result;
use application_owl::interactors::use_case_auth::AuthInteractor;
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::Response,
};
use infrastructure_owl::repository_impl::employee_repository_impl::EmployeeRepositoryImpl;
use std::{str::FromStr, sync::Arc};
use worker::Env;

use crate::{
    contollers::auth_controller::AuthController,
    presenters::auth_presenter::AuthPresenter,
    requests::{AuthProcessCode, AuthRequest},
};

#[worker::send]
pub async fn handle_auth(
    Path(auth_process_code): Path<String>, // 処理区分
    State(env): State<Arc<Env>>,           // 環境変数
    Json(auth_request): Json<AuthRequest>, // 認証リクエスト
) -> Response {
    let repository = EmployeeRepositoryImpl::new(env.clone());
    let presenter = AuthPresenter::new();
    let interactor = AuthInteractor::new(repository, presenter);
    let controller = AuthController::new(interactor);

    let result: Result<Response> =
        if let Ok(process) = AuthProcessCode::from_str(&auth_process_code) {
            match process {
                AuthProcessCode::Login => controller.login(auth_request).await,
                AuthProcessCode::Logout => controller.logout(auth_request).await,
                AuthProcessCode::Check => controller.check_auth(auth_request).await,
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

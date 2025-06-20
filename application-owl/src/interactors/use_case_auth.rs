use anyhow::Result;
use domain_owl::{
    domain_services::employee_service::EmployeeService,
    repositories::employee_repository::EmployeeRepository,
};

use crate::{
    application_errors::ApplicationAuthError, dtos::auth_dto::AuthRequestDTO,
    dtos::auth_dto::RegisterRequestDTO, input_ports::AuthInputPort, output_ports::AuthOutputPort,
};
use axum::response::Response;
use std::sync::Arc;

pub struct AuthInteractor<R: EmployeeRepository + Send + Sync + 'static, O: AuthOutputPort> {
    repository: Arc<R>,
    output_port: O,
    employee_service: EmployeeService<R>,
}

impl<R: EmployeeRepository + Send + Sync + 'static, O: AuthOutputPort> AuthInteractor<R, O> {
    pub fn new(repository: R, output_port: O) -> Self {
        let repository = Arc::new(repository);
        let employee_service = EmployeeService::new(Arc::clone(&repository));
        Self {
            repository,
            output_port,
            employee_service,
        }
    }
}

impl<R: EmployeeRepository + Send + Sync + 'static, O: AuthOutputPort> AuthInputPort
    for AuthInteractor<R, O>
{
    async fn login(&self, auth_request: AuthRequestDTO) -> Result<Response> {
        // idでユーザー取得
        let employee = match self.repository.find_by_id(&auth_request.employee_id).await {
            Some(employee) => employee,
            None => {
                return Ok(self
                    .output_port
                    .login_failure(ApplicationAuthError::NotFound.to_string().as_str()));
            }
        };
        // パスワード検証
        if !self
            .employee_service
            .verify_password(&auth_request.employee_id, &auth_request.password)
            .await
        {
            return Ok(self
                .output_port
                .login_failure(ApplicationAuthError::InvalidPassword.to_string().as_str()));
        }
        // ログイン可否判定
        if let Err(e) = self.employee_service.assert_can_login(&employee) {
            return Ok(self
                .output_port
                .login_failure(ApplicationAuthError::Domain(e).to_string().as_str()));
        }
        Ok(self.output_port.login_success())
    }

    async fn logout(&self, _auth_request: AuthRequestDTO) -> Result<Response> {
        Ok(self.output_port.logout_success())
    }

    async fn check_auth(&self, _auth_request: AuthRequestDTO) -> Result<Response> {
        Ok(self.output_port.auth_check_success(true))
    }

    async fn register(&self, register_request: RegisterRequestDTO) -> Result<Response> {
        match self
            .employee_service
            .register_new_employee(&register_request.employee_email, &register_request.password)
            .await
        {
            Ok(_) => Ok(self.output_port.register_success()),
            Err(e) => Ok(self.output_port.register_failure(&e.to_string())),
        }
    }
}

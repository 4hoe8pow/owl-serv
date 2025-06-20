use anyhow::Result;
use domain_owl::{
    domain_services::employee_service::EmployeeService,
    repositories::employee_repository::EmployeeRepository,
};

use crate::{
    application_errors::ApplicationAuthError, dtos::auth_dto::AuthRequestDTO,
    input_ports::AuthInputPort, output_ports::AuthOutputPort,
};
use axum::response::Response;

pub struct AuthInteractor<R: EmployeeRepository, O: AuthOutputPort> {
    repository: R,
    output_port: O,
    employee_service: EmployeeService<R>,
}

impl<R: EmployeeRepository + Clone, O: AuthOutputPort> AuthInteractor<R, O> {
    pub fn new(repository: R, output_port: O) -> Self {
        let employee_service = EmployeeService::new(repository.clone());
        Self {
            repository,
            output_port,
            employee_service,
        }
    }
}

impl<R: EmployeeRepository, O: AuthOutputPort> AuthInputPort for AuthInteractor<R, O> {
    async fn login(&self, auth_request: AuthRequestDTO) -> Result<Response> {
        // emailでユーザー取得
        let employee = match self
            .repository
            .find_by_email(&auth_request.employeename)
            .await
        {
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
            .verify_password(&employee, &auth_request.password)
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
}

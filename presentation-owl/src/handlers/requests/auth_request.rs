use std::str::FromStr;

use application_owl::dtos::auth_dto::{AuthRequestDTO, RegisterRequestDTO};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub employee_id: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email_address: String,
    pub password: String,
    pub department_id: i64,
    pub employee_name: String,
}

impl From<AuthRequest> for AuthRequestDTO {
    fn from(val: AuthRequest) -> Self {
        AuthRequestDTO {
            employee_id: val.employee_id,
            password: val.password,
        }
    }
}

impl From<RegisterRequest> for RegisterRequestDTO {
    fn from(val: RegisterRequest) -> Self {
        RegisterRequestDTO {
            email_address: val.email_address,
            password: val.password,
            department_id: val.department_id,
            employee_name: val.employee_name,
        }
    }
}

#[derive(Debug)]
pub enum AuthProcessCode {
    Login,
    Logout,
    Check,
    Register,
}

impl FromStr for AuthProcessCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" => Ok(AuthProcessCode::Login),
            "2" => Ok(AuthProcessCode::Logout),
            "3" => Ok(AuthProcessCode::Check),
            "4" => Ok(AuthProcessCode::Register),
            _ => Err(()),
        }
    }
}

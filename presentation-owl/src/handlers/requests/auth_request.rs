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
    pub employee_email: String,
    pub password: String,
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
            employee_email: val.employee_email,
            password: val.password,
        }
    }
}

#[derive(Debug)]
pub enum AuthProcessCode {
    Login,
    Logout,
    Check,
    Register, // 追加
}

impl FromStr for AuthProcessCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" => Ok(AuthProcessCode::Login),
            "2" => Ok(AuthProcessCode::Logout),
            "3" => Ok(AuthProcessCode::Check),
            "4" => Ok(AuthProcessCode::Register), // 追加
            _ => Err(()),
        }
    }
}

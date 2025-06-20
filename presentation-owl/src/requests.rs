use std::str::FromStr;

use application_owl::dtos::auth_dto::AuthRequestDTO;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthRequest {
    pub employee_id: String,
    pub employee_name: String,
    pub password: String,
}

impl From<AuthRequest> for AuthRequestDTO {
    fn from(val: AuthRequest) -> Self {
        AuthRequestDTO {
            employeename: val.employee_name,
            password: val.password,
        }
    }
}

#[derive(Debug)]
pub enum AuthProcessCode {
    Login,
    Logout,
    Check,
}

impl FromStr for AuthProcessCode {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "1" => Ok(AuthProcessCode::Login),
            "2" => Ok(AuthProcessCode::Logout),
            "3" => Ok(AuthProcessCode::Check),
            _ => Err(()),
        }
    }
}

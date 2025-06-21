use crate::domain_errors::DomainEmployeeError;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmployeeStatusCode {
    Hired,           // 入社
    PasswordChanged, // パスワード変更
    NameChanged,     // 氏名変更
    Suspended,       // 休職
    Reinstated,      // 復職
    Resigned,        // 退社
}

impl From<EmployeeStatusCode> for i64 {
    fn from(code: EmployeeStatusCode) -> Self {
        match code {
            EmployeeStatusCode::Hired => 1,
            EmployeeStatusCode::PasswordChanged => 2,
            EmployeeStatusCode::NameChanged => 3,
            EmployeeStatusCode::Suspended => 4,
            EmployeeStatusCode::Reinstated => 5,
            EmployeeStatusCode::Resigned => 6,
        }
    }
}

impl TryFrom<i64> for EmployeeStatusCode {
    type Error = DomainEmployeeError;
    fn try_from(id: i64) -> Result<Self, Self::Error> {
        match id {
            1 => Ok(EmployeeStatusCode::Hired),
            2 => Ok(EmployeeStatusCode::PasswordChanged),
            3 => Ok(EmployeeStatusCode::NameChanged),
            4 => Ok(EmployeeStatusCode::Suspended),
            5 => Ok(EmployeeStatusCode::Reinstated),
            6 => Ok(EmployeeStatusCode::Resigned),
            _ => Err(DomainEmployeeError::InvalidStatus),
        }
    }
}

impl From<EmployeeStatusCode> for &'static str {
    fn from(code: EmployeeStatusCode) -> Self {
        match code {
            EmployeeStatusCode::Hired => "HIRED",
            EmployeeStatusCode::PasswordChanged => "PASSWORD_CHANGED",
            EmployeeStatusCode::NameChanged => "NAME_CHANGED",
            EmployeeStatusCode::Suspended => "SUSPENDED",
            EmployeeStatusCode::Reinstated => "REINSTATED",
            EmployeeStatusCode::Resigned => "RESIGNED",
        }
    }
}

impl TryFrom<&str> for EmployeeStatusCode {
    type Error = DomainEmployeeError;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s.to_uppercase().as_str() {
            "HIRED" => Ok(EmployeeStatusCode::Hired),
            "PASSWORD_CHANGED" => Ok(EmployeeStatusCode::PasswordChanged),
            "NAME_CHANGED" => Ok(EmployeeStatusCode::NameChanged),
            "SUSPENDED" => Ok(EmployeeStatusCode::Suspended),
            "REINSTATED" => Ok(EmployeeStatusCode::Reinstated),
            "RESIGNED" => Ok(EmployeeStatusCode::Resigned),
            _ => Err(DomainEmployeeError::InvalidStatus),
        }
    }
}

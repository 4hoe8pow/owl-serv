use crate::domain_errors::DomainEmployeeError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EmployeeStatus {
    Active,    // 在職中
    Retired,   // 退職済み
    Suspended, // 休職中
}

impl std::str::FromStr for EmployeeStatus {
    type Err = DomainEmployeeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "在職中" | "active" => Ok(EmployeeStatus::Active),
            "退職済み" | "retired" => Ok(EmployeeStatus::Retired),
            "休職中" | "suspended" => Ok(EmployeeStatus::Suspended),
            _ => Err(DomainEmployeeError::InvalidStatus),
        }
    }
}

impl EmployeeStatus {
    pub fn new(s: &str) -> Result<Self, DomainEmployeeError> {
        s.parse()
    }
    pub fn value(&self) -> &str {
        match self {
            EmployeeStatus::Active => "在職中",
            EmployeeStatus::Retired => "退職済み",
            EmployeeStatus::Suspended => "休職中",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmployeeStatusCode {
    Nyusha,         // 入社
    PasswordChange, // パスワード変更
    NameChange,     // 氏名変更
    Kyushoku,       // 休職
    Fukushoku,      // 復職
    Taisha,         // 退社
}

impl EmployeeStatusCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            EmployeeStatusCode::Nyusha => "NYUSHA",
            EmployeeStatusCode::PasswordChange => "PASSWORD_CHANGE",
            EmployeeStatusCode::NameChange => "NAME_CHANGE",
            EmployeeStatusCode::Kyushoku => "KYUSHOKU",
            EmployeeStatusCode::Fukushoku => "FUKUSHOKU",
            EmployeeStatusCode::Taisha => "TAISHA",
        }
    }
}

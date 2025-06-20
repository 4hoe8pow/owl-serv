use crate::domain_errors::DomainEmployeeError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum EmployeeRole {
    General,
    PowerUser,
    Admin,
}

impl std::str::FromStr for EmployeeRole {
    type Err = DomainEmployeeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "general" => Ok(EmployeeRole::General),
            "poweruser" | "power_user" | "power" => Ok(EmployeeRole::PowerUser),
            "admin" => Ok(EmployeeRole::Admin),
            _ => Err(DomainEmployeeError::InvalidRole),
        }
    }
}

impl EmployeeRole {
    pub fn new(s: &str) -> Result<Self, DomainEmployeeError> {
        s.parse()
    }
    pub fn value(&self) -> &str {
        match self {
            EmployeeRole::General => "General",
            EmployeeRole::PowerUser => "PowerUser",
            EmployeeRole::Admin => "Admin",
        }
    }
}

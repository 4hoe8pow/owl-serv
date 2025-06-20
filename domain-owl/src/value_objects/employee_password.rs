use crate::domain_errors::DomainEmployeeError;
use serde::{Deserialize, Serialize};

const MIN_PASSWORD_LEN: usize = 9;
const MAX_PASSWORD_LEN: usize = 16;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmployeePassword(String);

impl EmployeePassword {
    pub fn new(s: &str) -> Result<Self, DomainEmployeeError> {
        let len = s.chars().count();
        if !(MIN_PASSWORD_LEN..=MAX_PASSWORD_LEN).contains(&len) {
            return Err(DomainEmployeeError::InvalidPasswordFormat);
        }
        let has_upper = s.chars().any(|c| c.is_ascii_uppercase());
        let has_lower = s.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = s.chars().any(|c| c.is_ascii_digit());
        let has_symbol = s.chars().any(|c| !c.is_ascii_alphanumeric());
        if has_upper && has_lower && has_digit && has_symbol {
            Ok(EmployeePassword(s.to_string()))
        } else {
            Err(DomainEmployeeError::InvalidPasswordFormat)
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EmployeePassword {
    fn from(s: &str) -> Self {
        EmployeePassword(s.to_string())
    }
}

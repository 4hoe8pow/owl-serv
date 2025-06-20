use crate::domain_errors::DomainEmployeeError;
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmployeeEmail(String);

impl EmployeeEmail {
    pub fn new(s: &str) -> Result<Self, DomainEmployeeError> {
        let re = Regex::new(r"^[\w\.-]+@[\w\.-]+\.[a-zA-Z]{2,}$").unwrap();
        if re.is_match(s) {
            Ok(EmployeeEmail(s.to_string()))
        } else {
            Err(DomainEmployeeError::InvalidEmailFormat)
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

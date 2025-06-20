use serde::{Deserialize, Serialize};
use thiserror::Error;

const MAX_NAME_BYTES: usize = 64;
const MIN_NAME_BYTES: usize = 1;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmployeeName(String);

#[derive(Error, Debug)]
pub enum EmployeeNameError {
    #[error("Name must be 1 to 64 bytes")]
    InvalidLength,
}

impl EmployeeName {
    pub fn new(s: &str) -> Result<Self, EmployeeNameError> {
        let len = s.len();
        if (MIN_NAME_BYTES..=MAX_NAME_BYTES).contains(&len) {
            Ok(EmployeeName(s.to_string()))
        } else {
            Err(EmployeeNameError::InvalidLength)
        }
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

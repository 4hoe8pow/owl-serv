use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeStatus(String);

impl EmployeeStatus {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EmployeeStatus {
    fn from(s: &str) -> Self {
        EmployeeStatus(s.to_string())
    }
}

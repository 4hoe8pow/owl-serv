use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeePassword(String);

impl EmployeePassword {
    pub fn value(&self) -> &str {
        &self.0
    }
}

impl From<&str> for EmployeePassword {
    fn from(s: &str) -> Self {
        EmployeePassword(s.to_string())
    }
}

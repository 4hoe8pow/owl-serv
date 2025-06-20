use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeEmail(String);

impl From<&str> for EmployeeEmail {
    fn from(s: &str) -> Self {
        EmployeeEmail(s.to_string())
    }
}

impl EmployeeEmail {
    pub fn value(&self) -> &str {
        &self.0
    }
}

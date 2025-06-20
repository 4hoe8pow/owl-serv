use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeId(String);

impl From<&str> for EmployeeId {
    fn from(s: &str) -> Self {
        EmployeeId(s.to_string())
    }
}

impl EmployeeId {
    pub fn value(&self) -> &str {
        &self.0
    }
}

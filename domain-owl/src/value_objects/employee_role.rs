use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeRole(String);

impl From<&str> for EmployeeRole {
    fn from(s: &str) -> Self {
        EmployeeRole(s.to_string())
    }
}

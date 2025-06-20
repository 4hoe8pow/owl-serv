use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct EmployeeName(String);

impl From<&str> for EmployeeName {
    fn from(s: &str) -> Self {
        EmployeeName(s.to_string())
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DepartmentName(pub String);

impl DepartmentName {
    pub fn new(name: &str) -> Self {
        DepartmentName(name.to_string())
    }
    pub fn value(&self) -> &str {
        &self.0
    }
}

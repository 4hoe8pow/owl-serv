use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct DepartmentId(pub i64);

impl DepartmentId {
    pub fn new(id: i64) -> Self {
        DepartmentId(id)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
}

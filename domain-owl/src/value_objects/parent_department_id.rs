use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ParentDepartmentId(pub i64);

impl ParentDepartmentId {
    pub fn new(id: i64) -> Self {
        ParentDepartmentId(id)
    }
    pub fn value(&self) -> i64 {
        self.0
    }
}

use crate::domain_errors::DomainEmployeeError;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct EmployeeId(Uuid);

impl std::str::FromStr for EmployeeId {
    type Err = DomainEmployeeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Uuid::parse_str(s)
            .map(EmployeeId)
            .map_err(|_| DomainEmployeeError::InvalidUuid)
    }
}

impl EmployeeId {
    pub fn new(id: Uuid) -> Self {
        EmployeeId(id)
    }
    pub fn value(&self) -> &Uuid {
        &self.0
    }
    pub fn from_str(s: &str) -> Result<Self, DomainEmployeeError> {
        s.parse()
    }
}

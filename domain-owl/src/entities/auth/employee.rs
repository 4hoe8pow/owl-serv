use serde::{Deserialize, Serialize};

use crate::value_objects::{
    employee_email::EmployeeEmail, employee_id::EmployeeId, employee_name::EmployeeName,
    employee_password::EmployeePassword, employee_role::EmployeeRole,
    employee_status::EmployeeStatus,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Employee {
    pub employee_id: EmployeeId,
    pub employee_name: EmployeeName,
    pub employee_email: EmployeeEmail,
    pub employee_password: EmployeePassword,
    pub employee_role: EmployeeRole,
    pub employee_status: EmployeeStatus,
}

impl Employee {
    pub fn new(
        employee_id: EmployeeId,
        employee_name: EmployeeName,
        employee_email: EmployeeEmail,
        employee_password: EmployeePassword,
        employee_role: EmployeeRole,
        employee_status: EmployeeStatus,
    ) -> Self {
        Self {
            employee_id,
            employee_name,
            employee_email,
            employee_password,
            employee_role,
            employee_status,
        }
    }
    /// JSON rowからEmployeeを生成（値オブジェクトのバリデーションを通す）
    pub fn from_row(row: &serde_json::Value) -> Option<Self> {
        Some(Employee {
            employee_id: EmployeeId::from_str(row.get("employee_id")?.as_str()?).ok()?,
            employee_name: EmployeeName::new(row.get("employee_name")?.as_str()?).ok()?,
            employee_email: EmployeeEmail::new(row.get("employee_email")?.as_str()?).ok()?,
            employee_password: EmployeePassword::new(row.get("employee_password")?.as_str()?)
                .ok()?,
            employee_role: EmployeeRole::new(row.get("employee_role")?.as_str()?).ok()?,
            employee_status: EmployeeStatus::new(row.get("employee_status")?.as_str()?).ok()?,
        })
    }
}

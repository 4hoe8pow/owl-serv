use serde::{Deserialize, Serialize};

use crate::value_objects::{
    department_id::DepartmentId, email_address::EmployeeEmail, employee_id::EmployeeId,
    employee_name::EmployeeName, employee_password::EmployeePassword, employee_role::EmployeeRole,
    employee_status::EmployeeStatusCode,
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Employee {
    pub employee_id: EmployeeId,
    pub employee_name: EmployeeName,
    pub email_address: EmployeeEmail,
    pub employee_password: EmployeePassword,
    pub employee_role: EmployeeRole,
    pub employee_status: EmployeeStatusCode,
    pub department_id: DepartmentId,
}

impl Employee {
    pub fn new(
        employee_id: EmployeeId,
        employee_name: EmployeeName,
        email_address: EmployeeEmail,
        employee_password: EmployeePassword,
        employee_role: EmployeeRole,
        employee_status: EmployeeStatusCode,
        department_id: DepartmentId,
    ) -> Self {
        Self {
            employee_id,
            employee_name,
            email_address,
            employee_password,
            employee_role,
            employee_status,
            department_id,
        }
    }
    /// JSON rowからEmployeeを生成（値オブジェクトのバリデーションを通す）
    pub fn from_row(row: &serde_json::Value) -> Option<Self> {
        let employee_id = EmployeeId::from_str(row.get("employee_id")?.as_str()?).ok()?;
        let employee_name = EmployeeName::new(row.get("employee_name")?.as_str()?).ok()?;
        let email_address = EmployeeEmail::new(row.get("email_address")?.as_str()?).ok()?;
        let employee_password = EmployeePassword::from(row.get("hashed_password")?.as_str()?);
        let employee_role = EmployeeRole::new(row.get("role")?.as_str()?).ok()?;
        let employee_status =
            EmployeeStatusCode::try_from(row.get("employment_status_id")?.as_i64()?).ok()?;
        let department_id = DepartmentId::new(row.get("department_id")?.as_i64()?);
        Some(Employee {
            employee_id,
            employee_name,
            email_address,
            employee_password,
            employee_role,
            employee_status,
            department_id,
        })
    }
}

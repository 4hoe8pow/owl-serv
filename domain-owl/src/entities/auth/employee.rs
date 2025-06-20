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
}

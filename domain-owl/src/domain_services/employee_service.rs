use crate::domain_errors::DomainEmployeeError;
use crate::entities::auth::employee::Employee;
use crate::repositories::employee_repository::EmployeeRepository;

pub struct EmployeeService<R: EmployeeRepository> {
    pub repository: R,
}

impl<R: EmployeeRepository> EmployeeService<R> {
    pub fn new(repository: R) -> Self {
        EmployeeService { repository }
    }

    pub fn verify_password(&self, employee: &Employee, raw_password: &str) -> bool {
        employee.employee_password.value() == raw_password
    }

    pub fn is_account_locked(&self, employee: &Employee) -> bool {
        employee.employee_status.value() == "locked"
    }

    pub fn is_email_verified(&self, employee: &Employee) -> bool {
        employee.employee_status.value() == "verified"
    }

    pub fn assert_can_login(&self, employee: &Employee) -> Result<(), DomainEmployeeError> {
        if self.is_account_locked(employee) {
            return Err(DomainEmployeeError::AccountLocked);
        }
        if !self.is_email_verified(employee) {
            return Err(DomainEmployeeError::EmailUnverified);
        }
        Ok(())
    }
}

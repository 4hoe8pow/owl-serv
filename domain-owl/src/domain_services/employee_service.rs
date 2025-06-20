use crate::domain_errors::DomainEmployeeError;
use crate::entities::auth::employee::Employee;
use crate::repositories::employee_repository::EmployeeRepository;
use crate::value_objects::{
    employee_email::EmployeeEmail, employee_id::EmployeeId, employee_name::EmployeeName,
    employee_password::EmployeePassword, employee_role::EmployeeRole,
    employee_status::EmployeeStatus,
};
use log::{debug, info};
use std::sync::Arc;
use uuid::Uuid;

pub struct EmployeeService<R: EmployeeRepository + Send + Sync> {
    repository: Arc<R>,
}

impl<R: EmployeeRepository + Send + Sync> EmployeeService<R> {
    pub fn new(repository: Arc<R>) -> Self {
        EmployeeService { repository }
    }

    pub async fn verify_password(&self, employee_id: &str, raw_password: &str) -> bool {
        self.repository
            .verify_password(employee_id, raw_password)
            .await
    }

    pub fn is_account_locked(&self, employee: &Employee) -> bool {
        employee.employee_status.value() == "locked"
    }

    pub fn assert_can_login(&self, employee: &Employee) -> Result<(), DomainEmployeeError> {
        if self.is_account_locked(employee) {
            return Err(DomainEmployeeError::AccountLocked);
        }
        Ok(())
    }

    pub async fn register_new_employee(
        &self,
        email: &str,
        password: &str,
    ) -> Result<(), DomainEmployeeError> {
        info!(
            "register_new_employee called: email={email}, password=***"
        );
        // email重複チェック
        debug!("Checking for duplicate email: {email}");
        if self.repository.find_by_email(email).await.is_some() {
            debug!("Duplicate email found: {email}");
            return Err(DomainEmployeeError::EmailAlreadyExists);
        }
        // パスワードバリデーション
        debug!("Validating password for email: {email}");
        let password_vo = match EmployeePassword::new(password) {
            Ok(vo) => vo,
            Err(e) => {
                debug!("Password validation failed: {e:?}");
                return Err(e);
            }
        };
        // パスワードハッシュ化
        debug!("Hashing password for email: {email}");
        let hashed = match self.repository.hash_password(password_vo.value()).await {
            Ok(h) => h,
            Err(e) => {
                debug!("Password hashing failed: {e:?}");
                return Err(DomainEmployeeError::InvalidPasswordFormat);
            }
        };
        // 必要な値オブジェクト生成
        debug!("Generating value objects for new employee");
        let employee_id = EmployeeId::new(Uuid::new_v4());
        let employee_name = EmployeeName::new("新規社員").unwrap(); // 仮
        let employee_email = EmployeeEmail::new(email)?;
        let employee_role = EmployeeRole::new("General").unwrap();
        let employee_status = EmployeeStatus::new("在職中").unwrap();
        let employee_password = EmployeePassword::from(hashed.as_str());
        debug!("Creating Employee struct");
        let employee = Employee::new(
            employee_id,
            employee_name,
            employee_email,
            employee_password,
            employee_role,
            employee_status,
        );
        debug!("Saving new employee to repository: email={email}");
        self.repository.save(&employee).await;
        info!(
            "register_new_employee finished successfully: email={email}"
        );
        Ok(())
    }
}

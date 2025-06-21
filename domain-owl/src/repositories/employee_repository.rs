use crate::entities::auth::employee::Employee;
use crate::owl_error::OwlError;

pub trait EmployeeRepository {
    async fn find_by_id(&self, id: &str) -> Option<Employee>;
    async fn save(&self, employee: &Employee) -> Result<(), Box<dyn OwlError>>;
    async fn delete(&self, id: i64);
    async fn verify_password(&self, employee_id: &str, raw_password: &str) -> bool;
    async fn find_by_email(&self, email: &str) -> Option<Employee>;
    async fn hash_password(&self, plain: &str) -> Result<String, String>;
}

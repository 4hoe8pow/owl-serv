use crate::entities::auth::employee::Employee;

pub trait EmployeeRepository {
    async fn find_by_email(&self, email: &str) -> Option<Employee>;
    async fn find_by_id(&self, id: i64) -> Option<Employee>;
    async fn save(&self, employee: &Employee);
    async fn delete(&self, id: i64);
}

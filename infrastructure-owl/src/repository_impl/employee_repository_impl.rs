use domain_owl::{
    entities::auth::employee::Employee, repositories::employee_repository::EmployeeRepository,
};
use std::sync::Arc;
use worker::Env;

#[derive(Clone)]
pub struct EmployeeRepositoryImpl {
    database_id: String,
}

impl EmployeeRepositoryImpl {
    pub fn new(_env: Arc<Env>) -> Self {
        let database_id = "mock_database_id".to_string();
        Self { database_id }
    }
}

impl EmployeeRepository for EmployeeRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Option<Employee> {
        // 仮実装: emailが"employee"なら返す
        if email == "employee" {
            Some(Employee::new(
                "test_id".into(),
                "test_employee".into(),
                "employee".into(),
                "pass".into(),
                "employee".into(),
                "verified".into(),
            ))
        } else {
            None
        }
    }
    async fn find_by_id(&self, _id: i64) -> Option<Employee> {
        None // 仮実装
    }
    async fn save(&self, _employee: &Employee) {
        // 仮実装: 何もしない
    }
    async fn delete(&self, _id: i64) {
        // 仮実装: 何もしない
    }
}

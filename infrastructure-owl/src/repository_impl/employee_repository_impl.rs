use domain_owl::{
    entities::auth::employee::Employee, repositories::employee_repository::EmployeeRepository,
};
use serde_json::Value;
use std::sync::Arc;
use worker::{Env, d1::D1Database, wasm_bindgen::JsValue};

const FIND_BY_EMAIL_SQL: &str = include_str!("../../sql/employee/find_by_email.sql");
const FIND_BY_ID_SQL: &str = include_str!("../../sql/employee/find_by_id.sql");
const SAVE_SQL: &str = include_str!("../../sql/employee/save.sql");
const DELETE_SQL: &str = include_str!("../../sql/employee/delete.sql");

pub struct EmployeeRepositoryImpl {
    d1: D1Database,
}

impl EmployeeRepositoryImpl {
    pub fn new(env: Arc<Env>) -> Self {
        let d1 = env.d1("DB").expect("D1 binding 'DB' not found");
        Self { d1 }
    }
}

impl EmployeeRepository for EmployeeRepositoryImpl {
    async fn find_by_email(&self, email: &str) -> Option<Employee> {
        let stmt = self.d1.prepare(FIND_BY_EMAIL_SQL);
        let row_opt: Option<Value> = stmt
            .bind(&[JsValue::from(email)])
            .ok()?
            .first(None)
            .await
            .ok()?;
        row_opt.and_then(|v| row_to_employee(&v))
    }

    async fn find_by_id(&self, id: i64) -> Option<Employee> {
        let stmt = self.d1.prepare(FIND_BY_ID_SQL);
        let id_str = id.to_string();
        let row_opt: Option<Value> = stmt
            .bind(&[JsValue::from(id_str)])
            .ok()?
            .first(None)
            .await
            .ok()?;
        row_opt.and_then(|v| row_to_employee(&v))
    }

    async fn save(&self, employee: &Employee) {
        let stmt = self.d1.prepare(SAVE_SQL);
        let id = employee.employee_id.value().to_string();
        let name = employee.employee_name.value().to_string();
        let email = employee.employee_email.value().to_string();
        let role = employee.employee_role.value().to_string();
        let params = [
            JsValue::from(id),
            JsValue::from(name),
            JsValue::from(email),
            JsValue::from(employee.employee_password.value()),
            JsValue::from(role),
            JsValue::from(employee.employee_status.value()),
        ];
        if let Ok(b) = stmt.bind(&params) {
            let _ = b.run().await;
        }
    }

    async fn delete(&self, id: i64) {
        let stmt = self.d1.prepare(DELETE_SQL);
        let id_str = id.to_string();
        if let Ok(b) = stmt.bind(&[JsValue::from(id_str)]) {
            let _ = b.run().await;
        }
    }
}

fn row_to_employee(row: &Value) -> Option<Employee> {
    Some(Employee::new(
        row.get("employee_id")?.as_str()?.into(),
        row.get("employee_name")?.as_str()?.into(),
        row.get("employee_email")?.as_str()?.into(),
        row.get("employee_password")?.as_str()?.into(),
        row.get("employee_role")?.as_str()?.into(),
        row.get("employee_status")?.as_str()?.into(),
    ))
}

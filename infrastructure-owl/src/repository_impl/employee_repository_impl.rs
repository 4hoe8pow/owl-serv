use crate::common::DatetimeUtils;
use crate::infrastructure_errors::InfrastructureAuthError;
use chrono::Utc;
use domain_owl::{
    entities::auth::employee::Employee, owl_error::OwlError,
    repositories::employee_repository::EmployeeRepository,
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
    datetime_utils: DatetimeUtils,
}

impl EmployeeRepositoryImpl {
    pub fn new(env: Arc<Env>) -> Self {
        let d1 = env.d1("DB").expect("D1 binding 'DB' not found");
        let datetime_utils = DatetimeUtils::new(env.clone());
        Self { d1, datetime_utils }
    }
}

impl EmployeeRepository for EmployeeRepositoryImpl {
    async fn find_by_id(&self, id: &str) -> Option<Employee> {
        let stmt = self.d1.prepare(FIND_BY_ID_SQL);
        let row_opt: Option<Value> = stmt
            .bind(&[JsValue::from(id)])
            .ok()? // SQLバインド失敗
            .first(None)
            .await
            .ok()?; // SQL実行失敗
        row_opt.and_then(|v| Employee::from_row(&v))
    }

    async fn save(&self, employee: &Employee) -> Result<(), Box<dyn OwlError>> {
        let now = Utc::now().naive_utc();
        let date_id = self.datetime_utils.get_or_insert_date_id(now.date()).await;
        let time_id = self.datetime_utils.get_or_insert_time_id(now.time()).await;
        if date_id.is_none() || time_id.is_none() {
            return Err(Box::new(InfrastructureAuthError::DateTimeIdError));
        }
        let stmt = self.d1.prepare(SAVE_SQL);
        let params = [
            JsValue::from(date_id.unwrap().to_string()), // date_id
            JsValue::from(time_id.unwrap().to_string()), // time_id
            JsValue::from(employee.employee_id.value().to_string()), // employee_id
            JsValue::from(employee.employee_name.value()), // employee_name
            JsValue::from(employee.department_id.value().to_string()), // department_id
            JsValue::from(i64::from(employee.employee_status).to_string()), // employment_status_id
            JsValue::from(employee.email_address.value()), // email_address
            JsValue::from(employee.employee_role.value()), // role
            JsValue::from(""),                           // comments（空文字）
            JsValue::from(employee.employee_password.value()), // hashed_password
        ];
        match stmt.bind(&params) {
            Ok(b) => {
                let result = b.run().await;
                match result {
                    Ok(_) => Ok(()),
                    Err(_) => Err(Box::new(InfrastructureAuthError::SaveFailed)),
                }
            }
            Err(_) => Err(Box::new(InfrastructureAuthError::BindFailed)),
        }
    }

    async fn delete(&self, id: i64) {
        let stmt = self.d1.prepare(DELETE_SQL);
        let id_str = id.to_string();
        if let Ok(b) = stmt.bind(&[JsValue::from(id_str.clone())]) {
            let _ = b.run().await;
        }
    }

    async fn verify_password(&self, employee_id: &str, raw_password: &str) -> bool {
        if let Some(employee) = self.find_by_id(employee_id).await {
            let hash = employee.employee_password.value();
            return bcrypt::verify(raw_password, hash).unwrap_or(false);
        }
        false
    }

    async fn find_by_email(&self, email: &str) -> Option<Employee> {
        let stmt = self.d1.prepare(FIND_BY_EMAIL_SQL);
        let row_opt: Option<Value> = stmt
            .bind(&[JsValue::from(email)])
            .ok()? // SQLバインド失敗
            .first(None)
            .await
            .ok()?; // SQL実行失敗
        row_opt.and_then(|v| Employee::from_row(&v))
    }

    async fn hash_password(&self, plain: &str) -> Result<String, String> {
        bcrypt::hash(plain, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())
    }
}

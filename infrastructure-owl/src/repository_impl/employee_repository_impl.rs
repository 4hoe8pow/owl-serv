use crate::common::DatetimeUtils;
use chrono::Utc;
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
            .ok()?
            .first(None)
            .await
            .ok()?;
        row_opt.and_then(|v| Employee::from_row(&v))
    }

    async fn save(&self, employee: &Employee) {
        // 現在日時を取得
        let now = Utc::now().naive_utc();
        let date_id = self.datetime_utils.get_or_insert_date_id(now.date()).await;
        let time_id = self.datetime_utils.get_or_insert_time_id(now.time()).await;
        if date_id.is_none() || time_id.is_none() {
            // 日付または時刻IDが取得できなければ保存しない
            return;
        }
        let stmt = self.d1.prepare(SAVE_SQL);
        let id = employee.employee_id.value().to_string();
        let name = employee.employee_name.value().to_string();
        let email = employee.employee_email.value().to_string();
        let role = employee.employee_role.value().to_string();
        let params = [
            JsValue::from(date_id.unwrap()),
            JsValue::from(time_id.unwrap()),
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
            .ok()?
            .first(None)
            .await
            .ok()?;
        row_opt.and_then(|v| Employee::from_row(&v))
    }

    async fn hash_password(&self, plain: &str) -> Result<String, String> {
        bcrypt::hash(plain, bcrypt::DEFAULT_COST).map_err(|e| e.to_string())
    }
}

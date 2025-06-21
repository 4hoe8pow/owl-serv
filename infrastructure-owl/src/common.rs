use chrono::{Datelike, NaiveDate, NaiveTime, Timelike, Weekday};
use std::sync::Arc;
use worker::{Env, d1::D1Database, wasm_bindgen::JsValue};

const SELECT_DATE_ID_SQL: &str = include_str!("../sql/utils/select_date_id.sql");
const INSERT_DATE_SQL: &str = include_str!("../sql/utils/insert_date.sql");
const SELECT_TIME_ID_SQL: &str = include_str!("../sql/utils/select_time_id.sql");
const INSERT_TIME_SQL: &str = include_str!("../sql/utils/insert_time.sql");

pub struct DatetimeUtils {
    d1: D1Database,
}

impl DatetimeUtils {
    pub fn new(env: Arc<Env>) -> Self {
        let d1 = env.d1("DB").expect("D1 binding 'DB' not found");
        Self { d1 }
    }

    async fn get_or_insert_id<F>(
        &self,
        select_sql: &str,
        insert_sql: &str,
        select_param: JsValue,
        insert_params: Vec<JsValue>,
        extract_id: F,
    ) -> Option<i64>
    where
        F: Fn(&serde_json::Value) -> Option<i64>,
    {
        // SELECT
        let stmt = self.d1.prepare(select_sql);
        if let Ok(b) = stmt.bind(&[select_param.clone()]) {
            if let Ok(Some(row)) = b.first::<serde_json::Value>(None).await {
                if let Some(id) = extract_id(&row) {
                    return Some(id);
                }
            }
        }
        // INSERT
        let stmt = self.d1.prepare(insert_sql);
        if let Ok(b) = stmt.bind(&insert_params) {
            let _ = b.run().await;
        }
        // 再度SELECT
        let stmt = self.d1.prepare(select_sql);
        if let Ok(b) = stmt.bind(&[select_param]) {
            if let Ok(Some(row)) = b.first::<serde_json::Value>(None).await {
                return extract_id(&row);
            }
        }
        None
    }

    pub async fn get_or_insert_date_id(&self, date: NaiveDate) -> Option<i64> {
        let date_str = date.format("%Y-%m-%d").to_string();
        let quarter = ((date.month0()) / 3) + 1;
        let is_weekend = match date.weekday() {
            Weekday::Sat | Weekday::Sun => 1,
            _ => 0,
        };
        let params = vec![
            JsValue::from(&date_str),
            JsValue::from(date.year()),
            JsValue::from(quarter),
            JsValue::from(date.month()),
            JsValue::from(date.day()),
            JsValue::from(date.weekday().number_from_monday()),
            JsValue::from(is_weekend),
        ];
        self.get_or_insert_id(
            SELECT_DATE_ID_SQL,
            INSERT_DATE_SQL,
            JsValue::from(&date_str),
            params,
            |row| row.get("date_id").and_then(|v| v.as_i64()),
        )
        .await
    }

    pub async fn get_or_insert_time_id(&self, time: NaiveTime) -> Option<i64> {
        let time_str = time.format("%H:%M:%S").to_string();
        let params = vec![
            JsValue::from(time.hour()),
            JsValue::from(time.minute()),
            JsValue::from(time.second()),
            JsValue::from(&time_str),
        ];
        self.get_or_insert_id(
            SELECT_TIME_ID_SQL,
            INSERT_TIME_SQL,
            JsValue::from(&time_str),
            params,
            |row| row.get("time_id").and_then(|v| v.as_i64()),
        )
        .await
    }
}

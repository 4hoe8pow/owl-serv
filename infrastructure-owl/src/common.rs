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

    pub async fn get_or_insert_date_id(&self, date: NaiveDate) -> Option<i64> {
        let date_str = date.format("%Y-%m-%d").to_string();
        // SELECT
        let stmt = self.d1.prepare(SELECT_DATE_ID_SQL);
        if let Ok(b) = stmt.bind(&[JsValue::from(&date_str)])
            && let Ok(Some(row)) = b.first::<serde_json::Value>(None).await
                && let Some(id) = row.get("date_id").and_then(|v| v.as_i64()) {
                    return Some(id);
                }
        // INSERT
        let quarter = ((date.month0()) / 3) + 1;
        let is_weekend = match date.weekday() {
            Weekday::Sat | Weekday::Sun => 1,
            _ => 0,
        };
        let params = [
            JsValue::from(&date_str),
            JsValue::from(date.year()),
            JsValue::from(quarter),
            JsValue::from(date.month()),
            JsValue::from(date.day()),
            JsValue::from(date.weekday().number_from_monday()),
            JsValue::from(is_weekend),
        ];
        let stmt = self.d1.prepare(INSERT_DATE_SQL);
        if let Ok(b) = stmt.bind(&params) {
            let _ = b.run().await;
        }
        // 再度SELECT
        let stmt = self.d1.prepare(SELECT_DATE_ID_SQL);
        if let Ok(b) = stmt.bind(&[JsValue::from(&date_str)])
            && let Ok(Some(row)) = b.first::<serde_json::Value>(None).await {
                return row.get("date_id").and_then(|v| v.as_i64());
            }
        None
    }

    pub async fn get_or_insert_time_id(&self, time: NaiveTime) -> Option<i64> {
        let time_str = time.format("%H:%M:%S").to_string();
        // SELECT
        let stmt = self.d1.prepare(SELECT_TIME_ID_SQL);
        if let Ok(b) = stmt.bind(&[JsValue::from(&time_str)])
            && let Ok(Some(row)) = b.first::<serde_json::Value>(None).await
                && let Some(id) = row.get("time_id").and_then(|v| v.as_i64()) {
                    return Some(id);
                }
        // INSERT
        let params = [
            JsValue::from(time.hour()),
            JsValue::from(time.minute()),
            JsValue::from(time.second()),
            JsValue::from(&time_str),
        ];
        let stmt = self.d1.prepare(INSERT_TIME_SQL);
        if let Ok(b) = stmt.bind(&params) {
            let _ = b.run().await;
        }
        // 再度SELECT
        let stmt = self.d1.prepare(SELECT_TIME_ID_SQL);
        if let Ok(b) = stmt.bind(&[JsValue::from(&time_str)])
            && let Ok(Some(row)) = b.first::<serde_json::Value>(None).await {
                return row.get("time_id").and_then(|v| v.as_i64());
            }
        None
    }
}

-- 次元テーブル
DROP TABLE IF EXISTS dim_date;
DROP TABLE IF EXISTS dim_time;
DROP TABLE IF EXISTS dim_vendor;
DROP TABLE IF EXISTS dim_device;
DROP TABLE IF EXISTS dim_account_title;
DROP TABLE IF EXISTS dim_department;
DROP TABLE IF EXISTS dim_project;
DROP TABLE IF EXISTS dim_employment_status;

-- ファクトテーブル
DROP TABLE IF EXISTS fact_auth;
DROP TABLE IF EXISTS fact_journal;
DROP TABLE IF EXISTS fact_cash_book;
DROP TABLE IF EXISTS fact_equity_book;
DROP TABLE IF EXISTS fact_employee;
DROP TABLE IF EXISTS fact_resource_assignment;

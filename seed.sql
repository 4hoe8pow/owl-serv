-- 初期化用シードSQL（Cloudflare D1 / SQLite3 互換）

PRAGMA defer_foreign_keys=TRUE;
-- 次元テーブル
DELETE FROM dim_date;
DELETE FROM dim_time;
DELETE FROM dim_vendor;
DELETE FROM dim_device;
DELETE FROM dim_account_title;
DELETE FROM dim_department;
DELETE FROM dim_project;
DELETE FROM dim_employment_status;

-- ファクトテーブル
DELETE FROM fact_auth;
DELETE FROM fact_journal;
DELETE FROM fact_cash_book;
DELETE FROM fact_equity_book;
DELETE FROM fact_employee;
DELETE FROM fact_resource_assignment;

-- --- dim_date 登録 ---
INSERT OR IGNORE INTO dim_date (
    calendar_date, year, quarter, month, day, day_of_week, is_weekend
) VALUES (
    '2025-06-21', 2025, 2, 6, 21, 6, 1
);

-- --- dim_time 登録 ---
INSERT OR IGNORE INTO dim_time (
    hour, minute, second, time_text
) VALUES (
    9, 0, 0, '09:00:00'
);

-- --- dim_department 登録 ---
INSERT OR IGNORE INTO dim_department (
    department_name
) VALUES
    ('管理部'),
    ('戦略推進部'),
    ('企画制作部');

-- --- dim_employment_status 登録 ---
INSERT OR IGNORE INTO dim_employment_status (
    employment_status_id, status_code, status_description
) VALUES
    (1, 'HIRED', '入社'),
    (2, 'PASSWORD_CHANGED', 'パスワード変更'),
    (3, 'NAME_CHANGED', '氏名変更'),
    (4, 'SUSPENDED', '休職'),
    (5, 'REINSTATED', '復職'),
    (6, 'RESIGNED', '退社');

-- --- IDの取得 ---

INSERT INTO fact_employee (
    date_id,
    time_id,
    employee_id,
    department_id,
    employment_status_id,
    email_address,
    role,
    comments,
    hashed_password
) VALUES (
    (SELECT date_id FROM dim_date WHERE calendar_date = '2025-06-21'),
    (SELECT time_id FROM dim_time WHERE time_text = '09:00:00'),
    1001,
    (SELECT department_id FROM dim_department WHERE department_name = '管理部'),
    (SELECT employment_status_id FROM dim_employment_status WHERE status_code = 'HIRED'),
    'tokunaga@jocarium.productions',
    '管理部マネージャー',
    '入社',
    'P@ssw0rd123'
);

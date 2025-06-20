-- Migration number: 0002 	 2025-06-20T15:52:51.079Z
-- 日付次元
CREATE TABLE dim_date (
    date_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    calendar_date TEXT    NOT NULL UNIQUE,        -- 'YYYY-MM-DD'
    year          INTEGER NOT NULL,
    quarter       INTEGER NOT NULL,
    month         INTEGER NOT NULL,
    day           INTEGER NOT NULL,
    day_of_week   INTEGER NOT NULL,               -- 0 (Sun) ～ 6 (Sat)
    is_weekend    INTEGER NOT NULL CHECK (is_weekend IN (0,1))
);

-- 時間次元
CREATE TABLE dim_time (
    time_id     INTEGER PRIMARY KEY AUTOINCREMENT,
    hour        INTEGER NOT NULL CHECK (hour BETWEEN 0 AND 23),
    minute      INTEGER NOT NULL CHECK (minute BETWEEN 0 AND 59),
    second      INTEGER NOT NULL CHECK (second BETWEEN 0 AND 59),
    time_text   TEXT    NOT NULL UNIQUE          -- 'HH:MM:SS'
);

-- 仕入先次元
CREATE TABLE dim_vendor (
    vendor_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    vendor_name    TEXT    NOT NULL UNIQUE,
    vendor_type    TEXT,
    contact_info   TEXT
);

-- デバイス次元
CREATE TABLE dim_device (
    device_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    device_name    TEXT    NOT NULL,
    device_type    TEXT,
    serial_number  TEXT    UNIQUE
);

-- 勘定科目次元
CREATE TABLE dim_account_title (
    account_title_id INTEGER PRIMARY KEY AUTOINCREMENT,
    code             TEXT    NOT NULL UNIQUE,
    title            TEXT    NOT NULL,
    category         TEXT
);

-- 部門次元
CREATE TABLE dim_department (
    department_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    department_name TEXT    NOT NULL UNIQUE,
    manager_id      INTEGER                         -- FK to employee_id if desired
);

-- プロジェクト次元
CREATE TABLE dim_project (
    project_id      INTEGER PRIMARY KEY AUTOINCREMENT,
    project_name    TEXT    NOT NULL UNIQUE,
    department_id   INTEGER                         -- FK to dim_department.department_id
        REFERENCES dim_department(department_id),
    start_date      INTEGER                         -- FK to dim_date.date_id
        REFERENCES dim_date(date_id),
    end_date        INTEGER                         -- FK to dim_date.date_id
        REFERENCES dim_date(date_id)
);

-- 雇用ステータス次元
CREATE TABLE dim_employment_status (
    employment_status_id INTEGER PRIMARY KEY AUTOINCREMENT,
    status_code         TEXT    NOT NULL UNIQUE,
    status_description  TEXT
);

-- 認証実績ファクト
CREATE TABLE fact_auth (
    auth_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id        INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    time_id        INTEGER NOT NULL
        REFERENCES dim_time(time_id),
    employee_id    INTEGER NOT NULL,
    auth_status    TEXT    NOT NULL,                -- e.g. 'SUCCESS', 'FAIL'
    ip_address     TEXT,
    user_agent     TEXT
);

-- 仕訳帳ファクト
CREATE TABLE fact_journal (
    journal_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id           INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    debit_account_id  INTEGER NOT NULL
        REFERENCES dim_account_title(account_title_id),
    credit_account_id INTEGER NOT NULL
        REFERENCES dim_account_title(account_title_id),
    amount            REAL    NOT NULL,
    description       TEXT
);

-- 現金入出金ファクト
CREATE TABLE fact_cash_book (
    cash_book_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id        INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    account_id     INTEGER NOT NULL
        REFERENCES dim_account_title(account_title_id),
    amount         REAL    NOT NULL,
    transaction_type TEXT  NOT NULL,                -- 'IN', 'OUT'
    description     TEXT
);

-- 持分台帳ファクト
CREATE TABLE fact_equity_book (
    equity_book_id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id        INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    account_id     INTEGER NOT NULL
        REFERENCES dim_account_title(account_title_id),
    amount         REAL    NOT NULL,
    owner          TEXT,
    description    TEXT
);

-- 人事履歴ファクト(社員マスタ)
CREATE TABLE fact_employee (
    employee_history_id INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id             INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    employee_id         INTEGER NOT NULL,
    department_id       INTEGER
        REFERENCES dim_department(department_id),
    status_id           INTEGER
        REFERENCES dim_employment_status(employment_status_id),
    role                TEXT,
    comments            TEXT
);

-- リソースアサイン実績ファクト
CREATE TABLE fact_resource_assignment (
    assignment_id  INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id        INTEGER NOT NULL
        REFERENCES dim_date(date_id),
    employee_id    INTEGER NOT NULL,
    project_id     INTEGER NOT NULL
        REFERENCES dim_project(project_id),
    assign_status  TEXT
);

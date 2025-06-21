-- Migration number: 0005 	 2025-06-21T15:31:53.392Z
DROP TABLE IF EXISTS fact_employee;

CREATE TABLE fact_employee (
    employee_event_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id             INTEGER NOT NULL,
    time_id             INTEGER NOT NULL,
    employee_id         INTEGER NOT NULL,
    department_id       INTEGER REFERENCES dim_department(department_id),
    status_code         INTEGER REFERENCES dim_employment_status(status_code),
    email_address       TEXT NOT NULL UNIQUE,
    role                TEXT,
    comments            TEXT,
    hashed_password     TEXT NOT NULL,
    FOREIGN KEY (date_id)      REFERENCES dim_date(date_id),
    FOREIGN KEY (time_id)      REFERENCES dim_time(time_id)
);

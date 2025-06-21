-- Migration number: 0007 	 2025-06-22T12:12:23.722Z
DROP TABLE IF EXISTS fact_employee;

CREATE TABLE fact_employee (
    employee_event_id    INTEGER PRIMARY KEY AUTOINCREMENT,
    date_id              INTEGER NOT NULL,
    time_id              INTEGER NOT NULL,
    employee_id          INTEGER NOT NULL,
    employee_name        TEXT NOT NULL,
    department_id        INTEGER REFERENCES dim_department(department_id),
    employment_status_id INTEGER REFERENCES dim_employment_status(employment_status_id),
    email_address        TEXT NOT NULL UNIQUE,
    role                 TEXT,
    comments             TEXT,
    hashed_password      TEXT NOT NULL,
    FOREIGN KEY (date_id)      REFERENCES dim_date(date_id),
    FOREIGN KEY (time_id)      REFERENCES dim_time(time_id)
);

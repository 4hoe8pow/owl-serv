-- Migration number: 0003 	 2025-06-20T23:39:41.266Z
DROP TABLE IF EXISTS fact_auth;

CREATE TABLE fact_session (
    session_id       INTEGER PRIMARY KEY AUTOINCREMENT,
    employee_id      INTEGER NOT NULL,
    session_token    TEXT    NOT NULL UNIQUE,
    refresh_token    TEXT    NOT NULL UNIQUE,
    issued_at        INTEGER NOT NULL,
    expires_at       INTEGER NOT NULL,
    revoked_at       INTEGER,
    FOREIGN KEY (employee_id) REFERENCES dim_employee(employee_id)
);

CREATE TABLE fact_auth (
    auth_id        INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id     INTEGER NOT NULL,
    date_id        INTEGER NOT NULL,
    time_id        INTEGER NOT NULL,
    auth_status    TEXT    NOT NULL,   -- e.g. 'SUCCESS', 'FAIL'
    ip_address     TEXT,
    machine_name   TEXT,
    FOREIGN KEY (session_id) REFERENCES fact_session(session_id),
    FOREIGN KEY (date_id) REFERENCES dim_date(date_id),
    FOREIGN KEY (time_id) REFERENCES dim_time(time_id)
);

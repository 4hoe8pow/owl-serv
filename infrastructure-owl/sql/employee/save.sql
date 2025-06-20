INSERT INTO fact_employee (
    date_id, time_id, employee_id, department_id, status_code, role, comments, hashed_password
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8
);

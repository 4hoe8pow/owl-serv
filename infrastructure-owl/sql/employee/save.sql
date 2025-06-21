INSERT INTO fact_employee (
    date_id, time_id, employee_id, employee_name, department_id, employment_status_id, email_address, role, comments, hashed_password
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10
);

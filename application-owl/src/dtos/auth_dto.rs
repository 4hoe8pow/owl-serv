pub struct AuthRequestDTO {
    pub employee_id: String,
    pub password: String,
}

pub struct RegisterRequestDTO {
    pub email_address: String,
    pub password: String,
    pub department_id: i64,    // 追加
    pub employee_name: String, // 追加
}

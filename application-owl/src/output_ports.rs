use axum::response::Response;

pub trait AuthOutputPort {
    fn login_success(&self) -> Response;
    fn login_failure(&self, message: &str) -> Response;
    fn logout_success(&self) -> Response;
    fn logout_failure(&self) -> Response;
    fn auth_check_success(&self, is_authenticated: bool) -> Response;
    fn auth_check_failure(&self) -> Response;
    fn register_success(&self) -> Response;
    fn register_failure(&self, message: &str) -> Response;
}

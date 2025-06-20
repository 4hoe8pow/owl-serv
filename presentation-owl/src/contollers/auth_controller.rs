use crate::requests::AuthRequest;
use anyhow::Result;
use application_owl::input_ports::AuthInputPort;
use axum::response::Response;

pub struct AuthController<I: AuthInputPort> {
    input_port: I,
}

impl<I: AuthInputPort> AuthController<I> {
    pub fn new(input_port: I) -> Self {
        AuthController { input_port }
    }

    pub async fn login(&self, auth_request: AuthRequest) -> Result<Response> {
        self.input_port.login(auth_request.into()).await
    }

    pub async fn logout(&self, auth_request: AuthRequest) -> Result<Response> {
        self.input_port.logout(auth_request.into()).await
    }

    pub async fn check_auth(&self, auth_request: AuthRequest) -> Result<Response> {
        self.input_port.check_auth(auth_request.into()).await
    }
}

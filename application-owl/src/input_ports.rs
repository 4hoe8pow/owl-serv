use crate::dtos::auth_dto::{AuthRequestDTO, RegisterRequestDTO};
use anyhow::Result;
use axum::response::Response;

pub trait AuthInputPort {
    async fn login(&self, auth_request: AuthRequestDTO) -> Result<Response>;
    async fn logout(&self, auth_request: AuthRequestDTO) -> Result<Response>;
    async fn check_auth(&self, auth_request: AuthRequestDTO) -> Result<Response>;
    async fn register(&self, register_request: RegisterRequestDTO) -> Result<Response>;
}

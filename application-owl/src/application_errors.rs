use domain_owl::domain_errors::DomainEmployeeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ApplicationAuthError {
    #[error("ユーザーが見つかりません")]
    NotFound,
    #[error("パスワードが正しくありません")]
    InvalidPassword,
    #[error(transparent)]
    Domain(#[from] DomainEmployeeError),
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainEmployeeError {
    #[error("アカウントがロックされています")]
    AccountLocked,
    #[error("メールアドレスが未認証です")]
    EmailUnverified,
}

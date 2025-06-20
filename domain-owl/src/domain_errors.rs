use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainEmployeeError {
    #[error("アカウントがロックされています")]
    AccountLocked,
    #[error("メールアドレスが未認証です")]
    EmailUnverified,
    #[error("メールアドレスの形式が不正です")]
    InvalidEmailFormat,
    #[error(
        "パスワードの形式が不正です（9-16文字、英大文字・小文字・数字・記号を含む必要があります）"
    )]
    InvalidPasswordFormat,
    #[error("ロールが不正です")]
    InvalidRole,
    #[error("ステータスが不正です")]
    InvalidStatus,
    #[error("UUIDが不正です")]
    InvalidUuid,
    #[error("メールアドレスが既に登録されています")]
    EmailAlreadyExists,
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureAuthError {
    #[error("DB接続エラー")]
    DatabaseError,
    // 必要に応じて追加
}

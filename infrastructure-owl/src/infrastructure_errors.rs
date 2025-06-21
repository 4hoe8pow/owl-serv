use thiserror::Error;

#[derive(Debug, Error)]
pub enum InfrastructureAuthError {
    #[error("DB接続エラー")]
    DatabaseError,
    #[error("日付IDまたは時刻IDの取得に失敗しました")]
    DateTimeIdError,
    #[error("パラメータバインドに失敗しました")]
    BindFailed,
    #[error("保存に失敗しました")]
    SaveFailed,
    // 必要に応じて追加
}

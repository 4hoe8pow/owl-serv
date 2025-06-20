// 認証認可(ログイン、ログアウト、ユーザー登録、パスワードリセット)
pub mod handle_auth;

// 勤怠管理(出退勤実績、残業申請)
pub mod handle_attendance;

// 財務会計(決算、仕訳、取引先)
pub mod handle_accounting;

// 資産管理(固定資産、棚卸資産、減価償却)
pub mod handle_asset;

// タレント管理(給与、賞与、人事異動、雇用イベントソーシング)
pub mod handle_talent;

// プロジェクト管理(タスク、進捗管理、アサイン、予算)
pub mod handle_project;

// ユーザ管理(アクセス権限管理)
pub mod handle_employee;

// 共通処理(PDF生成、CSV出力、メール送信、HTML生成、ログ管理)
pub mod handle_common;

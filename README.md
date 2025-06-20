# 🦉 Owl - ミニERPシステム

Owlは、Rust製の軽量なERP（Enterprise Resource Planning）バックエンドです。
Cloudflare Workers 上で稼働し、開発中は `wrangler` を用いたローカル実行が可能です。

---

## 🔧 前提条件

- Rust（1.74+ 推奨）
- [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/)（Cloudflare開発用CLI）
- Node.js（`npx` が使える環境）

---

## 🚀 ローカル開発手順

1. 依存のインストール（初回のみ）

```sh
cargo build
```

2. 開発サーバの起動

```sh
npx wrangler dev
```


http://localhost:8787

---

## 🏗️ CQRS＋イベントソーシング

本プロジェクトでは、更新系と参照系を明確に分離しつつ、全状態変化をイベントとして永続化するアーキテクチャを一部採用しています。

1. アーキテクチャ概要

- CQRS（Command–Query Responsibility Segregation）
  - コマンド（更新系）：ドメイン層のリポジトリトレイトの操作
  - クエリ（参照系）：専用の QueryServiceで最適化された検索・集計ロジックを実装
- イベントソーシング
  - すべての状態変化をイベントとして記録し、監査性・履歴再現性を保証
  - イベントの再生による集約再構築や外部システム連携を容易化

2. リポジトリ設計方針

- 単一 DB を前提にしつつ、アプリケーション層では書き込みモデルと読み取りモデルを論理的に分離
- ドメイン層のリポジトリトレイトは エンティティ単位 で一本化
- クエリの複雑化には、専用のリードモデル／DTO で対応
- 機能単位でリポジトリを乱立させるアンチパターンは避け、責務分離と一貫性を両立

3. イベントソーシングの適用と注意点

- イベントソーシングは実装や運用が複雑になりやすく、データ量やパフォーマンス、整合性維持などの課題も多い。
- すべてのドメインに万能ではなく、履歴管理や監査が重視される領域（金融取引、監査ログ、ワークフロー管理など）で特に有効。
- 単純なCRUDや履歴が不要な領域では従来型リポジトリパターンの方が適している場合も多い。

## 🛠 Database

開発と本番環境で使う D1 データベースの管理手順は以下の通りです：

### 1. 初期スキーマの反映（ローカル）

```bash
# 初回のスキーマ作成
npx wrangler d1 execute owl-database01 --local \
  --file=./infrastructure-owl/sql/create_schema.sql \
  --yes
```

- --local： ローカル SQLite DB に対して実行
- --yes： 確認プロンプトをスキップ

### 2. スキーマ変更の管理（マイグレーション）

```bash
# マイグレーションファイルの作成
npx wrangler d1 migrations create owl-database01 ${v00001_init_schema}

# マイグレーションのローカル適用
npx wrangler d1 migrations apply owl-database01 --local
```

### 3. プレビュー or 本番環境への適用

```bash
# Cloudflare に対して適用
npx wrangler d1 migrations apply owl-database01 --remote
```

### 4. 全体のexportが見たいとき

```bash
npx wrangler d1 export owl-database01 --output=schema.sql --remote --no-data
```

## お試し

```bash
curl -X POST http://localhost:8787/auth/1 \
  -H "Content-Type: application/json" \
  -d '{"employee_id":"12345","password":"supersecret"}'
```

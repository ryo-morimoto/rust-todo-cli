# 技術スタック

## 使用言語
- Rust (stable版)

## 必要なCrates

### コア機能
1. **clap** (v4.x)
   - コマンドライン引数のパース
   - サブコマンドの管理
   - 自動的なヘルプ生成
   - 学習ポイント: 派生マクロ、Builder パターン

2. **serde** (v1.x) + **serde_json** (v1.x)
   - JSONシリアライズ/デシリアライズ
   - データ永続化の実装
   - 学習ポイント: トレイトの実装、派生マクロ

3. **chrono** (v0.4.x)
   - 日付・時刻の扱い
   - タイムスタンプの管理
   - 学習ポイント: 外部クレートの使用方法

### UX向上
4. **colored** (v2.x)
   - ターミナルでのカラー出力
   - ステータスによる色分け
   - 学習ポイント: トレイトの拡張メソッド

5. **anyhow** (v1.x)
   - エラーハンドリングの簡潔化
   - コンテキスト付きエラー
   - 学習ポイント: Result型の実践的な使用

### 開発支援（オプション）
6. **directories** (v5.x)
   - クロスプラットフォーム対応のホームディレクトリ取得
   - 設定ファイルの適切な配置
   - 学習ポイント: OS依存処理の抽象化

## Cargo.toml 例

```toml
[package]
name = "rust-todo-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = { version = "0.4", features = ["serde"] }
colored = "2.1"
anyhow = "1.0"
directories = "5.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.1"
tempfile = "3.10"
```

## 選定理由

### 最小限の依存関係
- 学習目的のため、過度に複雑なcratesは避ける
- 各crateは特定の目的に特化しており、理解しやすい

### 実践的な学習
- 実際のRustプロジェクトでよく使われるcratesを採用
- エコシステムの標準的なパターンを学べる

### 段階的な学習
- 基本機能は標準ライブラリ中心で実装可能
- 必要に応じてcratesを追加していく設計

## 代替案

- **structopt** → clapのderive機能で十分
- **tokio** → 非同期処理は学習範囲外
- **diesel/sqlx** → JSONファイルで十分シンプル
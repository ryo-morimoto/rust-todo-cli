# Rust TODO CLI セットアップガイド

## 前提条件

### Rustのインストール
```bash
# Rustupを使ったインストール（推奨）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# インストール確認
rustc --version
cargo --version
```

### エディタ設定（推奨）
- VS Code + rust-analyzer拡張機能
- IntelliJ IDEA + Rustプラグイン

## プロジェクトの初期設定

### 1. プロジェクト作成
```bash
# プロジェクトディレクトリの作成
cargo new rust-todo-cli --bin
cd rust-todo-cli

# Gitリポジトリの初期化（既に完了済み）
git init
```

### 2. Cargo.tomlの設定
```bash
# Cargo.tomlを編集して依存関係を追加
```

`Cargo.toml`:
```toml
[package]
name = "rust-todo-cli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A simple TODO CLI application for learning Rust"
license = "MIT"

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

### 3. プロジェクト構造の作成
```bash
# srcディレクトリ構造
mkdir -p src/{models,commands,storage}

# テストディレクトリ
mkdir tests
```

推奨ディレクトリ構造:
```
rust-todo-cli/
├── Cargo.toml
├── Cargo.lock
├── README.md
├── requirement.md
├── techstack.md
├── setup.md
├── src/
│   ├── main.rs          # エントリーポイント
│   ├── lib.rs           # ライブラリルート
│   ├── models/          # データモデル
│   │   ├── mod.rs
│   │   └── todo.rs      # Todo構造体
│   ├── commands/        # CLIコマンド
│   │   ├── mod.rs
│   │   ├── add.rs       # 追加コマンド
│   │   ├── list.rs      # 一覧コマンド
│   │   ├── done.rs      # 完了コマンド
│   │   └── delete.rs    # 削除コマンド
│   └── storage/         # データ永続化
│       ├── mod.rs
│       └── json.rs      # JSON storage実装
└── tests/
    └── integration.rs   # 統合テスト
```

### 4. 基本ファイルの作成

#### src/main.rs
```rust
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // TODO: サブコマンドを追加
}

fn main() {
    println!("Hello, TODO CLI!");
}
```

#### src/lib.rs
```rust
pub mod models;
pub mod commands;
pub mod storage;
```

### 5. ビルドと実行確認
```bash
# 依存関係のダウンロードとビルド
cargo build

# 実行確認
cargo run

# リリースビルド
cargo build --release

# テスト実行
cargo test
```

## 開発フロー

### 1. 機能実装の流れ
1. モデル定義（`models/todo.rs`）
2. ストレージ実装（`storage/json.rs`）
3. コマンド実装（`commands/*.rs`）
4. CLIインターフェース統合（`main.rs`）
5. テスト作成

### 2. テスト駆動開発
```bash
# ユニットテストの実行
cargo test

# 特定のテストのみ実行
cargo test test_add_todo

# テスト出力を見る
cargo test -- --nocapture
```

### 3. コード品質チェック
```bash
# フォーマット
cargo fmt

# Lintチェック
cargo clippy

# ドキュメント生成
cargo doc --open
```

## デバッグとトラブルシューティング

### デバッグビルド
```bash
# デバッグ情報付きでビルド
cargo build

# 環境変数でログレベル設定
RUST_LOG=debug cargo run
```

### よくある問題

1. **依存関係の解決エラー**
   ```bash
   cargo clean
   cargo update
   ```

2. **バージョン互換性**
   - Rust 1.70以上を使用
   - `rustup update` で最新版に更新

3. **権限エラー（ファイル書き込み）**
   - ホームディレクトリの`.todo.json`に書き込み権限があるか確認

## 次のステップ

1. `src/models/todo.rs`でTodo構造体を実装
2. 基本的なCLIコマンドの実装
3. ファイル永続化の実装
4. エラーハンドリングの改善
5. カラー出力とUXの向上

## 参考リソース

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Clap Documentation](https://docs.rs/clap/latest/clap/)
- [Serde Documentation](https://serde.rs/)
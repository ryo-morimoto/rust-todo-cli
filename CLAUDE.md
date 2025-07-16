# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## プロジェクトの目的

このプロジェクトは、ユーザーがRustの構文や概念を理解するための学習用TODO CLIアプリケーションです。

**重要**: Claudeの責務は、コードを書くのではなく、利用する機能についての概要や詳細を回答することです。

## コミット規則

- このプロジェクトでは日本語でコミットメッセージを作成してください

## ビルドとテスト

```bash
# ビルド
cargo build
cargo build --release

# 実行
cargo run
cargo run -- <サブコマンド>

# テスト
cargo test
cargo test -- --nocapture  # テスト出力を表示
cargo test test_名前      # 特定のテストのみ実行

# コード品質
cargo fmt               # フォーマット
cargo clippy           # Lintチェック
cargo doc --open       # ドキュメント生成
```

## アーキテクチャ概要

### コマンド構造
- `todo add <タスク名>` - タスク追加
- `todo list` - タスク一覧表示
- `todo done <ID>` - タスク完了
- `todo delete <ID>` - タスク削除

### データモデル
- TODOタスクはID、タイトル、作成日時、ステータスを持つ
- データは`~/.todo.json`にJSON形式で永続化

### 主要な依存関係と学習ポイント

1. **clap** - コマンドライン引数パース
   - 派生マクロ、Builderパターンの学習

2. **serde/serde_json** - シリアライゼーション
   - トレイト実装、派生マクロの学習

3. **chrono** - 日時処理
   - 外部クレートの使用方法

4. **colored** - ターミナルカラー出力
   - トレイトの拡張メソッド

5. **anyhow** - エラーハンドリング
   - Result型の実践的使用

6. **directories** - クロスプラットフォーム対応
   - OS依存処理の抽象化

## Rust学習の重点項目

- 所有権システム
- Result型によるエラーハンドリング
- 構造体とトレイトの実装
- ファイルI/O
- CLIアプリケーションの設計パターン
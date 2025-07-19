# rust-todo-cli

Rustの学習用シンプルなTODO CLIアプリケーション

## インストール

```bash
# リポジトリをクローン
git clone https://github.com/ryo-morimoto/rust-todo-cli.git
cd rust-todo-cli

# ビルド & インストール
cargo install --path .

# または開発中は直接実行
cargo run -- <コマンド>
```

## 使い方

```bash
# タスクを追加
rtodo add "Rustを勉強する"

# タスク一覧を表示
rtodo list
rtodo list --all  # 完了済みも表示

# タスクを完了
rtodo done 1

# タスクを削除
rtodo delete 1
```

## 機能

- タスクの追加
- タスク一覧表示（アクティブ/完了済み）
- タスクの完了
- タスクの削除
- JSONファイルでの永続化（~/.todo.json）

## 開発

```bash
# テスト実行
cargo test

# フォーマット
cargo fmt

# Lintチェック
cargo clippy
```

## ライセンス

MIT
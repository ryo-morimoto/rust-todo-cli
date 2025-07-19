# Rustの学習ポイント

このプロジェクトを通じて学んだRustの重要な概念をまとめます。

## 1. 所有権システム

### 基本概念
- **所有権**: 値を所有する権利（1つの変数のみが持つ）
- **`&`**: 不変参照（読み取り専用の借用）
- **`&mut`**: 可変参照（読み書き可能な借用）
- **`mut`**: 変数を可変にする（所有権とは別の概念）

### プロジェクトでの実例

#### 不変参照の使用
```rust
// commands/list.rs
pub fn execute<R: TodoRepository>(repo: &R, all: bool) -> Result<()> {
    let todos = repo.find_all()?;  // repoは読み取りのみ
}
```

#### 可変参照の使用
```rust
// commands/add.rs
pub fn execute<R: TodoRepository>(repo: &mut R, title: String) -> Result<()> {
    repo.save(todo)?;  // repoの状態を変更
}
```

#### 所有権の移動
```rust
// todo.rs
pub fn complete(self) -> Result<Self, DomainError> {
    // selfの所有権を取得し、新しいインスタンスを返す
}
```

## 2. 可視性修飾子

### 階層構造
```rust
// プライベート（デフォルト）
field: String,

// 完全公開
pub field: String,

// クレート内公開
pub(crate) field: String,

// 親モジュールまで公開
pub(super) field: String,

// 特定パスまで公開
pub(in crate::models) field: String,
```

### 実践例
```rust
// models/todo.rs
pub struct Todo {
    pub(crate) id: TodoId,    // クレート内からアクセス可能
    title: NonEmptyString,     // プライベート
    pub status: TodoStatus,    // 完全公開
    pub created_at: DateTime<Local>,
}
```

## 3. トレイトと関連型

### トレイト境界
```rust
pub fn execute<R: TodoRepository>(repo: &mut R, title: String) -> Result<()>
//             ^^^^^^^^^^^^^^^^^ Rは TodoRepository を実装している型
```

### 関連型（Associated Types）
```rust
pub trait TodoRepository {
    type Error: std::error::Error + Send + Sync + 'static;
    // 実装ごとに異なるエラー型を定義可能
}
```

## 4. エラーハンドリング

### Result型と?演算子
```rust
fn execute() -> Result<()> {
    let id = repo.next_id()?;        // エラーを自動的に伝播
    let todo = Todo::new(id, title)?;
    repo.save(todo)?;
    Ok(())
}
```

### カスタムエラー型
```rust
#[derive(Debug)]
pub enum ValidationError {
    EmptyTitle,
}

impl std::error::Error for ValidationError {}
```

## 5. パターンマッチング

### Option型の処理
```rust
match repo.find_by_id(&todo_id)? {
    Some(todo) => {
        // TODOが見つかった場合の処理
    },
    None => {
        // 見つからなかった場合の処理
    }
}
```

### Enum型の処理
```rust
match todo.status {
    TodoStatus::Active => { /* アクティブ */ },
    TodoStatus::Completed { completed_at } => { /* 完了済み */ },
}
```

## 6. ジェネリクス

### 関数でのジェネリクス
```rust
pub fn execute<R: TodoRepository>(repo: &mut R, title: String) -> Result<()> {
    // Rは TodoRepository を実装している任意の型
}
```

## 7. ライフタイム

### 暗黙的なライフタイム
```rust
fn find_by_id(&self, id: &TodoId) -> Result<Option<Todo>, Self::Error>;
// &self と &TodoId のライフタイムは独立
```

## 8. テストの書き方

### ユニットテスト
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_todo_creation() {
        let todo = Todo::new(1, "タスク".to_string()).unwrap();
        assert_eq!(todo.id.value(), 1);
    }
}
```

### テスト用のモック
```rust
#[cfg(test)]
pub mod mock {
    // テスト時のみコンパイルされる
}
```

## まとめ

このプロジェクトを通じて、Rustの以下の重要な概念を実践的に学ぶことができました：

1. **所有権システム**: メモリ安全性を保証する仕組み
2. **借用チェッカー**: データ競合を防ぐ
3. **型システム**: コンパイル時の安全性
4. **エラーハンドリング**: Result型による明示的なエラー処理
5. **トレイト**: 抽象化とポリモーフィズム

これらの概念は、安全で効率的なシステムプログラミングの基礎となります。
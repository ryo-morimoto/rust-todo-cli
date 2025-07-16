# Neovim Rust開発環境セットアップガイド

このガイドでは、Ubuntu環境でNeovimを使ったRust開発環境を構築する手順を説明します。

## 前提条件
- Ubuntu OS
- インターネット接続

## Step 1: Neovimのインストール

```bash
# パッケージリストを更新
sudo apt update

# Neovimをインストール
sudo apt install neovim -y

# インストール確認
nvim --version
```

## Step 2: Rustのインストール

Rustupを使ってRustツールチェーンをインストールします。

```bash
# Rustupインストーラーをダウンロードして実行
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# プロンプトが表示されたら、デフォルト設定（1）を選択

# インストール後、環境変数を有効化
source "$HOME/.cargo/env"

# インストール確認
rustc --version
cargo --version
```

## Step 3: Neovim設定ディレクトリの準備

```bash
# Neovim設定ディレクトリを作成
mkdir -p ~/.config/nvim
```

## Step 4: 初期設定ファイルの作成

`~/.config/nvim/init.lua`を作成し、以下の内容を記述します：

```lua
-- lazy.nvimのインストール
local lazypath = vim.fn.stdpath("data") .. "/lazy/lazy.nvim"
if not vim.loop.fs_stat(lazypath) then
  vim.fn.system({
    "git",
    "clone",
    "--filter=blob:none",
    "https://github.com/folke/lazy.nvim.git",
    "--branch=stable",
    lazypath,
  })
end
vim.opt.runtimepath:prepend(lazypath)

-- プラグインの設定
require("lazy").setup({
  -- LSP設定
  {
    "neovim/nvim-lspconfig",
    dependencies = {
      "williamboman/mason.nvim",
      "williamboman/mason-lspconfig.nvim",
    },
  },
  
  -- 自動補完
  {
    "hrsh7th/nvim-cmp",
    dependencies = {
      "hrsh7th/cmp-nvim-lsp",
      "hrsh7th/cmp-buffer",
      "hrsh7th/cmp-path",
      "L3MON4D3/LuaSnip",
    },
  },
  
  -- シンタックスハイライト
  {
    "nvim-treesitter/nvim-treesitter",
    build = ":TSUpdate",
  },
  
  -- Rust専用ツール
  {
    "rust-lang/rust.vim",
    ft = "rust",
  },
})

-- 基本設定
vim.opt.number = true          -- 行番号表示
vim.opt.relativenumber = true  -- 相対行番号
vim.opt.expandtab = true       -- タブをスペースに
vim.opt.shiftwidth = 4         -- インデント幅
vim.opt.tabstop = 4            -- タブ幅
```

## Step 5: LSP（rust-analyzer）の設定

`~/.config/nvim/init.lua`の最後に以下を追加します：

```lua
-- LSPの設定
vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    require("mason").setup()
    require("mason-lspconfig").setup({
      ensure_installed = { "rust_analyzer" },
    })
    
    -- rust-analyzerの設定
    local lspconfig = require("lspconfig")
    lspconfig.rust_analyzer.setup({
      settings = {
        ["rust-analyzer"] = {
          cargo = {
            allFeatures = true,
          },
          checkOnSave = {
            command = "clippy",
          },
        },
      },
    })
  end,
})

-- 自動補完の設定
vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    local cmp = require("cmp")
    cmp.setup({
      snippet = {
        expand = function(args)
          require("luasnip").lsp_expand(args.body)
        end,
      },
      mapping = cmp.mapping.preset.insert({
        ["<C-b>"] = cmp.mapping.scroll_docs(-4),
        ["<C-f>"] = cmp.mapping.scroll_docs(4),
        ["<C-Space>"] = cmp.mapping.complete(),
        ["<C-e>"] = cmp.mapping.abort(),
        ["<CR>"] = cmp.mapping.confirm({ select = true }),
      }),
      sources = cmp.config.sources({
        { name = "nvim_lsp" },
        { name = "luasnip" },
      }, {
        { name = "buffer" },
        { name = "path" },
      }),
    })
  end,
})

-- Treesitterの設定
vim.api.nvim_create_autocmd("VimEnter", {
  callback = function()
    require("nvim-treesitter.configs").setup({
      ensure_installed = { "rust", "lua", "toml" },
      highlight = {
        enable = true,
      },
      indent = {
        enable = true,
      },
    })
  end,
})
```

## Step 6: 初回起動とプラグインのインストール

```bash
# Neovimを起動
nvim

# プラグインが自動的にインストールされます
# 完了後、一度Neovimを終了（:q）して再起動
```

## Step 7: 動作確認

サンプルのRustプロジェクトを作成して動作を確認します：

```bash
# サンプルプロジェクトを作成
cargo new hello_rust
cd hello_rust

# Neovimで編集
nvim src/main.rs
```

## 便利なキーバインド

- `<Space>` + `e`: エラー表示
- `gd`: 定義へジャンプ
- `gr`: 参照を検索
- `K`: ホバー情報表示
- `<Ctrl-Space>`: 補完候補表示
- `<CR>` (Enter): 補完確定

## トラブルシューティング

### rust-analyzerが動作しない場合

```bash
# rust-analyzerを手動でインストール
rustup component add rust-analyzer
```

### プラグインが読み込まれない場合

```bash
# Neovim内で以下のコマンドを実行
:Lazy sync
```

## 追加の推奨設定

より快適な開発環境のために、以下のツールもインストールすることをお勧めします：

```bash
# ripgrep（高速検索）
sudo apt install ripgrep

# fd（高速ファイル検索）
sudo apt install fd-find

# gitの設定（まだの場合）
git config --global user.name "Your Name"
git config --global user.email "your.email@example.com"
```

これで基本的なRust開発環境の構築は完了です！
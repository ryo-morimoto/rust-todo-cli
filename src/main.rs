use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // TODO: サブコマンドを追加
}

fn main() {
    println!("Hello, TODO CLI!");
}

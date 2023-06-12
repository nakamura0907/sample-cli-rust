extern crate sample_cli_rust;

fn main() {
    if let Err(e) = sample_cli_rust::run() {
        eprintln!("アプリケーションエラー: {}", e);
        std::process::exit(1);
    }
}

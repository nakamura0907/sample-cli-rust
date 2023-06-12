//! # Sample CLI Rust
//!
//! Sample CLI Rust は、Rustで作成したサンプルのRustCLIアプリケーションです。

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use std::{error::Error, process::Command};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Sample CLI Rust", about = "サンプルのRustCLIアプリケーション")]
struct Opt {
    /// デバッグモード
    #[structopt(short, long)]
    debug: bool,
    /// gitコマンド実行
    #[structopt(short, long)]
    execute: bool,
}
impl Opt {
    /// デバッグモードの場合にのみ実行する
    fn debug<F>(&self, f: F)
    where
        F: FnOnce(),
    {
        if self.debug {
            f();
        }
    }
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数の解析
    let opt = Opt::from_args();
    opt.debug(|| println!("{:#?}", opt));

    // プレフィックス選択
    let prefiexes = vec![
        "feat: 新機能追加",
        "fix: バグ修正",
        "refactor: コード修正",
        "chore: それ以外",
    ];

    let selected_prefix: Option<usize> = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("プレフィックスを選択してください")
        .items(&prefiexes)
        .default(0)
        .interact_on_opt(&Term::stderr())?;

    let prefix_index = match selected_prefix {
        Some(index) => index,
        None => return Err(From::from("プレフィックスが選択されていません")),
    };
    let prefix = prefiexes[prefix_index]
        .splitn(2, ":")
        .next()
        .unwrap()
        .trim()
        .to_string();

    // リファレンス（issue-1 または なし）入力
    let reference: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Option: リファレンス（e.g.issue-1）")
        .allow_empty(true)
        .interact_text()?;

    // 説明入力
    let description: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("ブランチの目的")
        .interact_text()?;

    // 開始ブランチ入力
    let start_branch: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Option: 開始ブランチ（e.g.origin/main）")
        .allow_empty(true)
        .interact_text()?;

    // ブランチ名生成
    let branch_name = if !reference.is_empty() {
        format!("{}/{}/{}", prefix, reference, description)
    } else {
        format!("{}/{}", prefix, description)
    };

    // gitコマンド生成
    let git_command = if !start_branch.is_empty() {
        format!("git checkout -b {} {}", branch_name, start_branch)
    } else {
        format!("git checkout -b {}", branch_name)
    };

    println!("コマンド: $ {}", git_command);

    // コマンド実行
    if !opt.execute {
        return Ok(());
    };

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", git_command.as_str()])
            .output()
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(git_command.as_str())
            .output()
    };
    if let Err(e) = output {
        return Err(From::from(format!("コマンド実行エラー: {}", e)));
    } else {
        println!("新しいブランチを作成しました: {}", branch_name);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;
}

//! # Sample CLI Rust
//!
//! Sample CLI Rust は、Rustで作成したサンプルのRustCLIアプリケーションです。

use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use std::{error::Error, process::Command};
use structopt::StructOpt;

pub fn run() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数解釈
    let opt = parse_cli_args();

    // gitコマンド生成
    let branch_info = input_branch_into()?;
    let branch_name = branch_info.generate_branch_name();
    let git_command = branch_info.generate_git_command();

    println!("コマンド: $ {}", git_command);

    // コマンド実行
    if !opt.execute {
        return Ok(());
    };
    execute_git_command(&branch_name, &git_command)?;

    Ok(())
}

/// コマンドライン引数
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

/// コマンドライン引数の解釈
fn parse_cli_args() -> Opt {
    let opt = Opt::from_args();
    opt.debug(|| println!("{:#?}", opt));

    opt
}

/// ブランチ情報
struct BranchInfo {
    prefix: String,
    reference: String,
    description: String,
    start_branch: String,
}
impl BranchInfo {
    /// ブランチ名を生成する
    fn generate_branch_name(&self) -> String {
        if !self.reference.is_empty() {
            format!("{}/{}/{}", self.prefix, self.reference, self.description)
        } else {
            format!("{}/{}", self.prefix, self.description)
        }
    }

    /// gitコマンドを生成する
    fn generate_git_command(&self) -> String {
        if !self.start_branch.is_empty() {
            format!(
                "git checkout -b {} {}",
                self.generate_branch_name(),
                self.start_branch
            )
        } else {
            format!("git checkout -b {}", self.generate_branch_name())
        }
    }
}

/// ブランチ情報をインタラクティブに入力
fn input_branch_into() -> Result<BranchInfo, Box<dyn Error>> {
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

    Ok(BranchInfo {
        prefix,
        reference,
        description,
        start_branch,
    })
}

/// gitコマンドを実行する
fn execute_git_command(branch_name: &str, git_command: &str) -> Result<(), Box<dyn Error>> {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", git_command]).output()
    } else {
        Command::new("sh").arg("-c").arg(git_command).output()
    };

    if let Err(e) = output {
        return Err(From::from(format!("コマンド実行エラー: {}", e)));
    }
    println!("新しいブランチを作成しました: {}", branch_name);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn test_branch_info() {
        let branch_info = super::BranchInfo {
            prefix: "feat".to_string(),
            reference: "issue-1".to_string(),
            description: "新機能追加".to_string(),
            start_branch: "origin/main".to_string(),
        };

        assert_eq!(
            branch_info.generate_branch_name(),
            "feat/issue-1/新機能追加"
        );
        assert_eq!(
            branch_info.generate_git_command(),
            "git checkout -b feat/issue-1/新機能追加 origin/main"
        );

        let branch_info_without_option = super::BranchInfo {
            prefix: "feat".to_string(),
            reference: "".to_string(),
            description: "新機能追加".to_string(),
            start_branch: "".to_string(),
        };

        assert_eq!(
            branch_info_without_option.generate_branch_name(),
            "feat/新機能追加"
        );
        assert_eq!(
            branch_info_without_option.generate_git_command(),
            "git checkout -b feat/新機能追加"
        );
    }
}

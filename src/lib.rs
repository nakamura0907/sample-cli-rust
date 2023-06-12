use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "サンプル CLI Rust", about = "サンプル CLI アプリケーション")]
struct Opt {
    /// デバッグモード
    #[structopt(short, long)]
    debug: bool,
}

pub fn run() -> Result<(), Box<dyn Error>> {
    // コマンドライン引数の解析
    let opt = Opt::from_args();

    if opt.debug {
        println!("{:#?}", opt);
    }

    // インタラクティブ対話モード

    Ok(())
}

#[cfg(test)]
mod test {
    // use super::*;
}

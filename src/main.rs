// エクスプローラー上からUnix系バイナリを実行
use anyhow::Context;
use anyhow::Result as Res;
use std::env::args;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
// メインエントリー
fn main() -> Res<()> {
    // 引数のリストの取得
    let args: Vec<String> = args().skip(1).collect();
    // パスの取得
    let path = &args[0];
    // パスからC:を取り除く
    let exec_path: String = path.chars().skip(2).collect();
    // パスをUnix形式に変換
    let unix_exec_path = "/mnt/c".to_owned() + &exec_path.replace("\\", "/");
    // パスを表示
    println!("unix path: {:?}", unix_exec_path);

    // unix_exec_pathのディレクトリ名だけ取得
    if let Some(parent) = Path::new(&unix_exec_path).parent() {
        // wslを起動

        let mut cmd = Command::new("wsl");
        let _ = cmd
            .args(&[
                "cd",
                &format!("{}", parent.display()),
                "&&",
                &unix_exec_path,
            ])
            .stdout(Stdio::piped())
            .spawn()?
            .wait()?;
    }
    // 入力の待機
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;

    Ok(())
}

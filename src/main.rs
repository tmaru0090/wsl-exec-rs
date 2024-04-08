use anyhow::Context;
use anyhow::Result as Res;
use std::env::args;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;
fn main() -> Res<()> {
    let args: Vec<String> = args().skip(1).collect();
    let path = &args[0];
    let exec_path: String = path.chars().skip(2).collect();
    let unix_exec_path = "/mnt/c".to_owned() + &exec_path.replace("\\","/");

    println!("path: {:?}",unix_exec_path);
    let mut cmd = Command::new("wsl");
    let _ = cmd.arg(unix_exec_path).stdout(Stdio::piped()).spawn()?.wait()?;
    let mut i = String::new();
    std::io::stdin().read_line(&mut i)?;

    Ok(())
}

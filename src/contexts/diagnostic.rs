use minijinja::context;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn get_hash() -> Option<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .ok()?; 

    if output.status.success() {
        let hash = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Some(hash)
    } else {
        eprintln!("Failed to get git commit hash: {}", String::from_utf8_lossy(&output.stderr));
        None
    }
}

fn get_compiler_info() -> Option<Vec<String>> {
    let output = Command::new("rustc")
        .args(["--version", "--verbose"])
        .output()
        .ok()?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<String> = output_str.lines().map(|line| line.to_string()).collect();
        Some(lines)
    } else {
        None
    }
}
fn get_timestamp() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs()
}

pub async fn context() -> minijinja::value::Value {
    let git_commit = get_hash().unwrap_or_else(|| "unknown".to_string());
    let build_time = get_timestamp();
    let compiler_info = get_compiler_info();
    context! {
        git_commit => git_commit,
        build_time => build_time,
        compiler_info => compiler_info
    }
}
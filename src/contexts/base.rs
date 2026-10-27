use minijinja::context;
use serde::Serialize;
use std::process::Command;

#[derive(Serialize)]
struct Commit {
    hash: String,
    short_hash: String,
    author: String,
    date: String,
    message: String,
}

pub async fn context() -> minijinja::value::Value {
    let out = Command::new("git")
        .args(["log", "-1", "--format=%H%x1f%h%x1f%an%x1f%aI%x1f%s"])
        .output()
        .expect("unable to run git");
    assert!(out.status.success(), "git log failed");

    let stdout = String::from_utf8(out.stdout).expect("git output wasn't utf8");
    let mut p = stdout.trim().split('\x1f').map(String::from);

    let commit = Commit {
        hash: p.next().unwrap_or_default(),
        short_hash: p.next().unwrap_or_default(),
        author: p.next().unwrap_or_default(),
        date: p.next().unwrap_or_default(),
        message: p.next().unwrap_or_default(),
    };

    context! {
        commit => commit
    }
}
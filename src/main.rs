mod builder;
mod contexts;

use crate::builder::{get_environment, build_all};

fn main() {
    let env = get_environment();
    let _ = build_all(&env);
}

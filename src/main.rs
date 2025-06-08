mod builder;
mod contexts;

use crate::builder::{generate_page, get_environment};

fn main() {
    let env = get_environment();
    println!("{}", generate_page(&env, "/tep"))
}

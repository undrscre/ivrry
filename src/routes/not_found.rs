use crate::get_env;
use minijinja::context;

pub async fn page_html() -> String {
    let env = get_env();
    let tmpl = env.get_template("not_found.html").unwrap();
    tmpl.render(context! {}).unwrap()
}
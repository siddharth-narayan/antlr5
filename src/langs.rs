use minijinja::Environment;

use crate::ast::ANTLRAst;

pub fn codegen(env: Environment, tree: ANTLRAst) {
    println!("{}", env.get_template("rust-parse").unwrap().render(tree).unwrap())
}

pub fn jinja_env() -> Environment<'static> {
    let mut env = Environment::new();

    env.add_template("rust-parse", include_str!("langs/rust/parser.jinja")).unwrap();
    env.add_template("rust-parse-alt", include_str!("langs/rust/alt.jinja")).unwrap();

    env
}

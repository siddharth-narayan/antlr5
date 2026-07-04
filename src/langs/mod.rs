use crate::antlr::ANTLRAst;
use minijinja::Environment;

pub fn codegen(env: Environment, tree: ANTLRAst) {
    println!("{}", env.get_template("rust-parse").unwrap().render(tree).unwrap())
}

pub fn jinja_env() -> Environment<'static> {
    let mut env = Environment::new();

    env.add_template("rust-parse", include_str!("rust/parser.jinja"));

    env
}
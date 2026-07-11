use minijinja::Environment;

use crate::{analysis::SymbolTable, ast::ANTLRAst};

pub fn codegen(env: Environment, tree: ANTLRAst) {
    println!("{}", env.get_template("rust-parse").unwrap().render(tree).unwrap())
}

pub fn jinja_env(symbols: SymbolTable) -> Environment<'static> {
    let mut env = Environment::new();

    // Env settings must be set above templates
    env.set_lstrip_blocks(true);
    env.set_trim_blocks(true);

    env.add_template("rust-parse", include_str!("langs/rust/parser.jinja")).unwrap();
    
    
    let closure = move | name: String | { 
        symbols.get_rule_id(name)
    };

    env.add_filter("startstate", closure);

    env
}



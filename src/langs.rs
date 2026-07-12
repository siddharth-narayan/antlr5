use minijinja::{Environment, UndefinedBehavior, Value, value::ViaDeserialize};
use serde::Deserializer;

use crate::{analysis::SymbolTable, ast::{ANTLRAst, ebnf::EBNFSuffix, rules::Element}};

pub fn codegen(env: Environment, tree: ANTLRAst) {
    println!("{}", env.get_template("rust-parse").unwrap().render(tree).unwrap())
}

pub fn jinja_env(symbols: SymbolTable) -> Environment<'static> {
    let mut env = Environment::new();

    // Env settings must be set above templates
    env.set_lstrip_blocks(true);
    env.set_trim_blocks(true);
    env.set_undefined_behavior(UndefinedBehavior::Chainable);

    env.add_template("rust-parse", include_str!("langs/rust/parser.jinja")).unwrap();
    
    
    let closure = move | name: String | { 
        symbols.get_rule_id(name)
    };

    let etype_prefix = | e: ViaDeserialize<Element> | {
        // let e: Element = e.deserialize_any();
        if let Some(suffix) = e.suffix() {
            match suffix {
                EBNFSuffix::Optional => "Option<".into(),
                EBNFSuffix::Plus | EBNFSuffix::Star => "Vec<".into(),
            }
        } else {
            String::new()
        }
    };

    let etype_suffix = | e: ViaDeserialize<Element> | {
        // let e: Element = e.deserialize_any();
        if let Some(suffix) = e.suffix() {
            ">".into()
        } else {
            String::new()
        }
    };

    println!("OIJWEOIWJE: Eleement ub: {:#?}", env.undefined_behavior());

    env.add_filter("capitalize", capitalize);
    env.add_filter("startstate", closure);
    env.add_filter("etype_prefix", etype_prefix);
    env.add_filter("etype_suffix", etype_suffix);

    env
}

pub fn capitalize(string: String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
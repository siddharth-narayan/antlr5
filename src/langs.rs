use std::{ffi::OsStr, fs, path::Path, process::Command, sync::Arc};

use minijinja::{Environment, UndefinedBehavior, Value, value::ViaDeserialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, alt::AltIR, element::{ElementIR}}, langs::filters::{capitalize, element_prefix, element_suffix, id_from_rule_name_filter, lookahead_lookup_filter, rule_from_id_filter, token_from_id_filter, uppercase}};

mod filters;

#[derive(Clone, Copy)]
pub enum Language {
    Rust,
    Python
}

pub fn render(ir: Arc<AntlrIR>, env: Environment, lang: Language) -> String {
    match lang {
        Language::Rust => {
            env.get_template("rust-parse").unwrap().render(ir).unwrap()
        },
        
        Language::Python => {
            env.get_template("python-parse").unwrap().render(ir).unwrap()
        }
    }
}

pub fn format<P: AsRef<OsStr>>(path: P, lang: Language) {
    match lang {
        Language::Rust => {
            Command::new("rustfmt").arg(path).output();
        },

        Language::Python => {

        }
    }
}

pub fn output<P: AsRef<Path> + AsRef<OsStr>>(ir: Arc<AntlrIR>, path: P, env: Environment, lang: Language) {
    let content = render(ir, env, lang);
    fs::write(&path, content);
    format(&path, lang);
}

pub fn jinja_env(ir: Arc<AntlrIR>) -> Environment<'static> {
    let mut env = Environment::new();

    // Env settings must be set above templates
    env.set_lstrip_blocks(true);
    env.set_trim_blocks(true);
    env.set_undefined_behavior(UndefinedBehavior::Chainable);

    env.add_template("rust-parse", include_str!("langs/rust/parser.jinja")).unwrap();
    env.add_template("rust-lookahead", include_str!("langs/rust/lookahead.jinja")).unwrap();
    env.add_template("python-parse", include_str!("langs/python/parser.jinja")).unwrap();
    
    add_default_filters(&mut env, ir);

    env
}

pub fn add_default_filters(env: &mut Environment, ir: Arc<AntlrIR>) {
    env.add_filter("capitalize", capitalize);
    env.add_filter("uppercase", uppercase);

    env.add_filter("etype_prefix", element_prefix);
    env.add_filter("etype_suffix", element_suffix);
    env.add_filter("etoken_type_prefix", element_prefix);
    env.add_filter("etoken_type_suffix", element_suffix);

    env.add_filter("rule_from_id", rule_from_id_filter(ir.clone()));
    env.add_filter("id_from_rule", id_from_rule_name_filter(ir.clone()));
    env.add_filter("token_from_id", token_from_id_filter(ir.clone()));
    env.add_filter("lookahead_lookup_filter", lookahead_lookup_filter(ir.clone()));
}
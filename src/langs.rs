use std::sync::Arc;

use minijinja::{Environment, UndefinedBehavior, Value, value::ViaDeserialize};

use crate::{antlr::ast::{ANTLRAst, EBNFSuffix, Element}, codegen::{intermediate::{AntlrIR, ElementIR, RuleIR, TokenElementIR}, symbols::SymbolTable}};

pub fn jinja_env(ir: Arc<AntlrIR>) -> Environment<'static> {
    let ir = ir.clone();

    let mut env = Environment::new();

    // Env settings must be set above templates
    env.set_lstrip_blocks(true);
    env.set_trim_blocks(true);
    env.set_undefined_behavior(UndefinedBehavior::Chainable);

    env.add_template("rust-parse", include_str!("langs/rust/parser.jinja")).unwrap();
    env.add_template("python-parse", include_str!("langs/python/parser.jinja")).unwrap();
    
    let ir_clone = ir.clone();
    let closure = move | name: String | { 
        ir_clone.symbols().get_rule_id(&name)
    };

    let etype_prefix = | e: ViaDeserialize<ElementIR> | {
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

    let etoken_type_prefix = | e: ViaDeserialize<ElementIR> | {
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

    let etype_suffix = | e: ViaDeserialize<ElementIR> | {
        // let e: Element = e.deserialize_any();
        if let Some(_suffix) = e.suffix() {
            ">".into()
        } else {
            String::new()
        }
    };

    let etoken_type_suffix = | e: ViaDeserialize<TokenElementIR> | {
        // let e: Element = e.deserialize_any();
        if let Some(_suffix) = e.suffix() {
            ">".into()
        } else {
            String::new()
        }
    };

    let ir_clone = ir.clone();
    let rule_from_id = move | id: usize | -> Option<Value> {
        let rule = ir_clone.rules().get(id)?.clone();
        Some(Value::from_serialize(rule))
    };

    let ir_clone = ir.clone();
    let tokenrule_from_id = move | id: usize | -> Option<Value> {
        let value = if let Some(rule) = ir_clone.token_rules().get(id) {
            Some(Value::from_serialize(rule.clone()))
        } else if let Some(token) = ir_clone.token_rules().get(id) {
            Some(Value::from_serialize(token.clone()))
        } else {
            None
        };

        value
    };

    env.add_filter("capitalize", capitalize);
    env.add_filter("startstate", closure);
 
    env.add_filter("etype_prefix", etype_prefix);
    env.add_filter("etype_prefix", etype_prefix);
    env.add_filter("etoken_type_prefix", etoken_type_prefix);
    env.add_filter("etoken_type_suffix", etoken_type_suffix);


    env.add_filter("etype_suffix", etype_suffix);
    env.add_filter("rule_from_id", rule_from_id);
    env.add_filter("token_from_id", tokenrule_from_id);

    env
}

pub fn capitalize(string: String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}
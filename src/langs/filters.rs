use std::sync::Arc;

use minijinja::{Value, value::ViaDeserialize};

use crate::{antlr::ast::EBNFSuffix, codegen::{analysis::match_rule, intermediate::{AntlrIR, alt::AltIR, element::ElementIR}}, util::HashArc};

pub fn element_prefix(e: ViaDeserialize<ElementIR>) -> String {
    if let Some(suffix) = e.suffix() {
        match suffix {
            EBNFSuffix::Optional => "Option<".into(),
            EBNFSuffix::Plus | EBNFSuffix::Star => "Vec<".into(),
        }
    } else {
        String::new()
    }
}

pub fn element_suffix(e: ViaDeserialize<ElementIR>) -> String {
    if let Some(_suffix) = e.suffix() {
        ">".into()
    } else {
        String::new()
    }
}

pub fn id_from_rule_name_filter(ir: Arc<AntlrIR>) -> impl Fn(String) -> Value {
    move | name: String | -> Value {
        let rule = ir.symbols().get_rule_id(&name).clone();
        Value::from_serialize(rule)
    }
}

pub fn id_from_tokenrule_name_filter(ir: Arc<AntlrIR>) -> impl Fn(String) -> Value {
    move | name: String | -> Value {
        let rule = ir.symbols().get_token_id(&name).clone();
        Value::from_serialize(rule)
    }
}

pub fn rule_from_id_filter(ir: Arc<AntlrIR>) -> impl Fn(usize) -> Option<Value> {
    move | id: usize | -> Option<Value> {
        let rule = ir.rules().get(id)?.clone();
        Some(Value::from_serialize(rule))
    }
}
pub fn token_from_id_filter(ir: Arc<AntlrIR>) -> impl Fn(usize) -> Option<String> {
    move | id: usize | -> Option<String> {
        ir.symbols().get_token_name(id)
    }
}

pub fn capitalize(string: String) -> String {
    let mut c = string.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_titlecase().collect::<String>() + c.as_str(),
    }
}

pub fn uppercase(string: String) -> String {
    string.to_uppercase()
}

// We DEEP clone the Arc<AntlrIR> here. Any further changes will not affect this specific lookup
pub fn lookahead(ir: Arc<AntlrIR>) -> impl Fn(usize) -> Option<Value> {
    move | rule | -> Option<Value> {
        let mut ir = Arc::unwrap_or_clone(ir.clone());
        Some(Value::from_serialize(match_rule(ir.into(), rule)))
    }
}
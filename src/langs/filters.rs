use std::sync::Arc;

use minijinja::{Value, value::ViaDeserialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, alt::AltIR, element::{ElementIR}}};

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
pub fn token_from_id_filter(ir: Arc<AntlrIR>) -> impl Fn(usize) -> Option<Value>{
    move | id: usize | -> Option<Value> {
        if let Some(rule) = ir.token_rules().get(id) {
            Some(Value::from_serialize(rule.clone()))
        } else if let Some(token) = ir.token_rules().get(id) {
            Some(Value::from_serialize(token.clone()))
        } else {
            None
        }
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
// pub fn lookahead_lookup_filter(ir: Arc<AntlrIR>) -> impl Fn(ViaDeserialize<Vec<Arc<AltIR>>>) -> Option<Value> {
//     move | alts: ViaDeserialize<Vec<Arc<AltIR>>> | -> Option<Value> {
//         let mut ir = Arc::unwrap_or_clone(ir.clone());
//         Some(Value::from_serialize(ir.internal_lookahead_alts(&alts)))
//     }
// }
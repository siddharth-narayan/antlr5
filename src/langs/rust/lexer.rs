use std::sync::Arc;

use crate::codegen::intermediate::AntlrIR;

pub fn lexer_match(ir: Arc<AntlrIR>) -> String {
    for token_rule in ir.token_rules() {
        
    };

    "// lexer match here".into()
}
use std::sync::Arc;

use crate::codegen::intermediate::AntlrIR;

pub fn lexer_tokens(ir: Arc<AntlrIR>) -> String {
    let enum_variants = ir.token_rules().iter().map(|t| {
        t.name().clone()
    }).collect::<Vec<_>>().join(",\n");

    format!(
        "#[derive(Clone, Copy, Debug)]
        pub enum TokenType {{
            {}
        }}", enum_variants
    )
} 
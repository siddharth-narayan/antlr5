use std::{collections::HashSet, sync::Arc};

use crate::{codegen::{analysis::alt_len, intermediate::AntlrIR}, tests::parse};

#[test]
pub fn altlen() {
    let grammar = 
        "grammar x;
        a: b c | b c d ;
        b: 'B';
        c: 'c';
        d: 'd';
        ";

    let ir = Arc::new(parse(grammar));

    // let mut visited = HashSet::new();
    let mut lens = HashSet::new();
    alt_len(ir.get_rule_alt(0, 0).unwrap(), 0, 0, &mut HashSet::new(), &mut lens, ir.rules());
    println!("Alt lens: {:#?}", lens);

    panic!()
}
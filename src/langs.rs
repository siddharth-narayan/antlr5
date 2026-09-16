use std::{ffi::OsStr, fs, path::Path, process::Command, sync::Arc, time::SystemTime};

use minijinja::{Environment, UndefinedBehavior, Value, value::ViaDeserialize};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, alt::AltIR, element::ElementIR}};

pub mod rust;

#[derive(Clone, Copy)]
pub enum Language {
    Rust,
    Python
}

pub fn render(ir: Arc<AntlrIR>, lang: Language) -> String {
    match lang {
        Language::Rust => {
            rust::render(ir)
        },
        
        Language::Python => {
            "unimplemented!".into()
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

pub fn output<P: AsRef<Path> + AsRef<OsStr>>(ir: Arc<AntlrIR>, path: P, lang: Language) {
    let time = SystemTime::now();
    let content = render(ir, lang);
    fs::write(&path, content);
    println!("Generated parser in {}ms", time.elapsed().unwrap().as_millis());
    format(&path, lang);
}
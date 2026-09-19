use std::{ffi::OsStr, fs, io::{self, Error}, path::{Path, PathBuf}, process::Command, sync::Arc, time::SystemTime};

use crate::{antlr::ast::EBNFSuffix, codegen::intermediate::{AntlrIR, alt::AltIR, element::ElementIR}};

pub mod rust;
pub mod c;

#[derive(Clone, Copy)]
pub enum Language {
    C,
    Rust,
    Python
}

pub struct OutputFile {
    path: PathBuf,
    content: String,
}

pub fn render<P: AsRef<Path>>(ir: Arc<AntlrIR>, lang: Language, path: P) -> Vec<OutputFile> {
    match lang {
        Language::C => c::render(ir, path.as_ref().into()),
        
        Language::Rust => rust::render(ir),
        
        Language::Python => {
            Vec::new()
        }
    }
}

pub fn format(path: PathBuf, lang: Language) {
    let result = match lang {
        Language::C => {
            Command::new("clang-format").arg("-i").arg("--style={ColumnLimit: 0}").arg(path.as_os_str()).output()
        },
        
        Language::Rust => {
            Command::new("rustfmt").arg(path.as_os_str()).output()
        },

        Language::Python => {
           io::Result::Err(Error::from_raw_os_error(2))
        }
    };

    if let Err(_) = result {
        println!("Failed to format your output. It will be messy! Ensure you have the approprate formatter installed for your target language")
    }
}

pub fn output(ir: Arc<AntlrIR>, path: PathBuf, lang: Language) {
    let time = SystemTime::now();
    
    let outputs = render(ir, lang, path);
    for output in &outputs {
        fs::write(&output.path, &output.content);
    }

    println!("Generated parser in {}ms", time.elapsed().unwrap().as_millis());

    let time = SystemTime::now();
    
    for output in &outputs {
        format(output.path.clone(), lang);
    }
    
    println!("Formatted parser in {}ms", time.elapsed().unwrap().as_millis());    
}
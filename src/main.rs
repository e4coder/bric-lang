extern crate llvm_sys as llvm;
#[macro_use]
extern crate lalrpop_util;

use std::fs;

mod ast;

fn main() {
    
    let source_result = fs::read_to_string("test.bric");
    let source = match source_result {
       Ok(s) => s,
       Err(_e) => r#"let x = "Could not load from test source";"#.to_string(), 
    };

    let program = ast::bric_lang::ProgramParser::new()
        .parse(&source)
        .expect("Unable to parse the program file");

    for statement in program.items {
        println!("{:?}",statement)
    }

}

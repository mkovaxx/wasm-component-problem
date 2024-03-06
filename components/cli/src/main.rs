#[allow(warnings)]
mod bindings;

use crate::bindings::example::parser::example_parser_parsing::parse;
use crate::bindings::example::types::types::Ast;

fn main() {
    let source = "stuff will go here";
    let ast: Ast = parse(source);
    println!("AST: {ast:?}");
}

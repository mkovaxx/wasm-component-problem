#[allow(warnings)]
mod bindings;

use crate::bindings::exports::example::parser::parsing::Guest;
use crate::bindings::example::types::types::Ast;

struct Component;

impl Guest for Component {
    fn parse(_source: String) -> Ast {
        Ast::Empty
    }
}

bindings::export!(Component with_types_in bindings);

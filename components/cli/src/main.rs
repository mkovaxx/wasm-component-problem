#[allow(warnings)]
mod bindings;

fn main() {
    let source = "stuff will go here";
    let ast: Ast = parse(source);
    println!("AST: {ast:?}");
}

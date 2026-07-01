use full_moon::{parse, visitors::VisitorMut};

use crate::obfuscator::mutations::numerics::NumericMutator;

mod numerics;

pub fn apply(input: &String) -> String {
    let ast = parse(input).expect("Failed to parse input ast");

    let mut binary_expression = NumericMutator;

    let mutated = binary_expression.visit_ast(ast);

    mutated.to_string()
}

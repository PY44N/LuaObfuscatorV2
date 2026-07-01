use full_moon::{parse, visitors::VisitorMut};

use crate::obfuscator::mutations::binary_expression::BinaryExpressionMutator;

mod binary_expression;

pub fn apply(input: &String) -> String {
    let ast = parse(input).expect("Failed to parse input ast");

    let mut binary_expression = BinaryExpressionMutator;

    let mutated = binary_expression.visit_ast(ast);

    mutated.to_string()
}

use full_moon::{parse, visitors::VisitorMut};

use crate::{
    obfuscation_settings::ObfuscationSettings, obfuscator::mutations::numerics::NumericMutator,
};

mod numerics;

pub fn apply(input: &String, settings: &ObfuscationSettings) -> String {
    let ast = parse(input).expect("Failed to parse input ast");

    if settings.constant_mutations {
        let mut binary_expression = NumericMutator;

        let mutated = binary_expression.visit_ast(ast);

        return mutated.to_string();
    }

    input.clone()
}

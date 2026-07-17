use full_moon::{parse, visitors::VisitorMut};

use crate::obfuscator::macros::crash::CrashMacroAdder;

pub mod crash;

static MACRO_PREFIX: &str = "LUOB_";

pub fn apply(input: &String) -> String {
    let ast = parse(input).expect("Failed to parse macro ast");

    let mut crash_adder = CrashMacroAdder;

    let crash_added = crash_adder.visit_ast(ast);

    return crash_added.to_string();
}

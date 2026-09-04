use full_moon::{ast::Stmt, parse, visitors::VisitorMut};

use crate::obfuscator::macros::MACRO_PREFIX;

pub struct CrashMacroAdder;

impl VisitorMut for CrashMacroAdder {
    // fn fold_expression(
    //     &mut self,
    //     node: &full_moon::ast::Expression,
    // ) -> Option<full_moon::ast::Expression> {
    //     println!("{:#?}", node);

    //     None

    //     // match node {
    //     //     full_moon::ast::Expression::FunctionCall(function_call) => {
    //     //         println!("{:#?}", function_call);

    //     //         None
    //     //     }
    //     //     _ => None,
    //     // }
    // }

    fn replace_function_call(
        &mut self,
        node: &full_moon::ast::FunctionCall,
    ) -> Option<full_moon::ast::FunctionCall> {
        let function_name = node.prefix().to_string().trim().to_string();

        if function_name == MACRO_PREFIX.to_owned() + "CRASH" {
            let parsed_function =
                parse("(nil)() ").expect("Failed to parse generated inline function");
            if let Some(Stmt::FunctionCall(f)) = parsed_function.nodes().stmts().next() {
                return Some(f.clone());
            }
        }

        None
    }
}

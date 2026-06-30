use std::collections::HashMap;

use full_moon::{
    ast::{self, Expression, Stmt},
    parse,
    visitors::VisitorMut,
};

struct CollectFunctionPass<'a> {
    function_map: &'a mut HashMap<String, ast::FunctionBody>,
}

impl VisitorMut for CollectFunctionPass<'_> {
    fn visit_block(&mut self, node: ast::Block) -> ast::Block {
        let new_statments: Vec<_> = node
            .stmts_with_semicolon()
            .filter(|(v, _)| {
                if let Stmt::LocalFunction(f) = v {
                    if f.name().to_string().contains("_INLINE") {
                        if self.function_map.contains_key(&f.name().to_string()) {
                            panic!("Function {} already defined", f.name());
                        }

                        self.function_map
                            .insert(f.name().to_string(), f.body().clone());

                        return false;
                    }
                }

                if let Stmt::LocalAssignment(a) = v {
                    let name = a.names().iter().next().unwrap().to_string();
                    if name.contains("_INLINE") {
                        let b = a.expressions().iter().next().unwrap();
                        if let Expression::Function(f) = b {
                            if self.function_map.contains_key(name.trim()) {
                                panic!("Function {} already defined", name);
                            }

                            self.function_map
                                .insert(name.trim().to_string(), f.body().clone());

                            return false;
                        }
                    }
                }

                true
            })
            .cloned()
            .collect();

        node.with_stmts(new_statments)
    }
}

struct InlineFunctionPass<'a> {
    function_map: &'a mut HashMap<String, ast::FunctionBody>,
}

impl VisitorMut for InlineFunctionPass<'_> {
    fn visit_function_call(&mut self, node: ast::FunctionCall) -> ast::FunctionCall {
        let function_name = node.prefix().to_string().trim().to_string();
        if let Some(body) = self.function_map.get(&function_name) {
            if let Some(ast::Suffix::Call(ast::Call::AnonymousCall(ac))) = node.suffixes().next() {
                let new_function = format!("(function{}){}", body.to_string().trim(), ac);

                let parsed_function =
                    parse(&new_function).expect("Failed to parse generated inline function");
                if let Some(Stmt::FunctionCall(f)) = parsed_function.nodes().stmts().next() {
                    return f.clone();
                }
            }

            panic!("Failed to replace function call")
        }

        node
    }
}

pub struct Inliner;

impl Inliner {
    pub fn inline(ast: ast::Ast) -> ast::Ast {
        let mut function_map = HashMap::new();

        let mut function_collector = CollectFunctionPass {
            function_map: &mut function_map,
        };

        let new_ast = function_collector.visit_ast(ast);

        let mut function_inliner = InlineFunctionPass {
            function_map: &mut function_map,
        };

        let newest_ast = function_inliner.visit_ast(new_ast);

        newest_ast
    }

    pub fn inline_from_code(code: String) -> String {
        let ast = parse(&code).expect("Failed to parse original ast");

        Self::inline(ast).to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nothing() {
        assert_eq!(
            Inliner::inline_from_code("print('hi')".to_string()),
            "print('hi')"
        );
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            Inliner::inline_from_code(
                "local function printer_INLINE(str) print(str) end printer_INLINE('Hi')"
                    .to_string()
            ),
            "(function(str) print(str) end)('Hi')"
        );
    }

    #[test]
    fn test_alternate_function_def() {
        assert_eq!(
            Inliner::inline_from_code(
                "local printer_INLINE = function(str) print(str) end printer_INLINE('Hi')"
                    .to_string()
            ),
            "(function(str) print(str) end)('Hi')"
        );
    }

    #[test]
    fn test_nested_call() {
        assert_eq!(
            Inliner::inline_from_code(
                "local function printer_INLINE(str) print(str) end local function add_INLINE(a, b) return a + b end printer_INLINE(add_INLINE(2, 4))"
                    .to_string()
            ),
            "(function(str) print(str) end)((function(a, b) return a + b end)(2, 4))"
        );
    }

    #[test]
    fn test_nested_definition() {
        assert_eq!(
            Inliner::inline_from_code(
                "local function add_INLINE(a, b) return a + b end local function print_add_INLINE(str) print(add_INLINE(str)) end print_add_INLINE(2, 4)"
                    .to_string()
            ),
            "(function(str) print((function(a, b) return a + b end)(str)) end)(2, 4)"
        );
    }
}

use std::fmt;

use full_moon::{ast::LastStmt, parse, visitors::VisitorMut};
use rand::{Rng, rng, seq::IndexedRandom};

use crate::randomizable_enum;

// TODO: Maybe mult + div (not sure if that would cause issues though)
randomizable_enum! {
    #[derive(Debug, Clone, Copy)]
    enum ArithmeticOperator {
        Add,
        Subract,
        Multiply,
        Divide
    }
}

impl ArithmeticOperator {
    fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            ArithmeticOperator::Add => a + b,
            ArithmeticOperator::Subract => a - b,
            ArithmeticOperator::Multiply => a * b,
            ArithmeticOperator::Divide => a / b,
        }
    }

    fn invert(&self) -> Self {
        match self {
            ArithmeticOperator::Add => ArithmeticOperator::Subract,
            ArithmeticOperator::Subract => ArithmeticOperator::Add,
            ArithmeticOperator::Multiply => ArithmeticOperator::Divide,
            ArithmeticOperator::Divide => ArithmeticOperator::Multiply,
        }
    }
}

impl fmt::Display for ArithmeticOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ArithmeticOperator::Add => "+",
                ArithmeticOperator::Subract => "-",
                ArithmeticOperator::Multiply => "*",
                ArithmeticOperator::Divide => "/",
            },
        )
    }
}

pub struct BinaryExpressionMutator;

impl VisitorMut for BinaryExpressionMutator {
    fn fold_expression(
        &mut self,
        node: &full_moon::ast::Expression,
    ) -> Option<full_moon::ast::Expression> {
        if let full_moon::ast::Expression::Number(num) = node {
            let number: f64 = num.token().to_string().parse().unwrap();

            let second_num: f64 = rng().random::<f64>() * 50.0;
            let op = ArithmeticOperator::random();

            let first_num = op.invert().apply(number, second_num);

            let encrypted_ast = parse(&format!("return ({} {} {})", first_num, op, second_num))
                .expect("Failed to parse generated binary expression code");

            let last_stmt = encrypted_ast
                .nodes()
                .last_stmt()
                .expect("Failed to extract last statement");

            if let LastStmt::Return(ret_stmt) = last_stmt {
                let expr = ret_stmt
                    .returns()
                    .iter()
                    .next()
                    .expect("Failed to get return expr");

                return Some(expr.clone());
            }

            panic!("Failed to find return statement")
        }

        None
    }
}

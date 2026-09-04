use std::fmt;

use full_moon::visitors::VisitorMut;
use rand::{Rng, rng, seq::IndexedRandom};

use crate::{obfuscator::utils, randomizable_enum};

// TODO: Maybe mult + div (not sure if that would cause issues though)
randomizable_enum! {
    #[derive(Debug, Clone, Copy)]
    enum ArithmeticOperator {
        Add,
        Subract,
        // Multiply,
        // Divide
    }
}

impl ArithmeticOperator {
    fn apply(&self, a: f64, b: f64) -> f64 {
        match self {
            ArithmeticOperator::Add => a + b,
            ArithmeticOperator::Subract => a - b,
            // ArithmeticOperator::Multiply => a * b,
            // ArithmeticOperator::Divide => a / b,
        }
    }

    fn invert(&self) -> Self {
        match self {
            ArithmeticOperator::Add => ArithmeticOperator::Subract,
            ArithmeticOperator::Subract => ArithmeticOperator::Add,
            // ArithmeticOperator::Multiply => ArithmeticOperator::Divide,
            // ArithmeticOperator::Divide => ArithmeticOperator::Multiply,
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
                // ArithmeticOperator::Multiply => "*",
                // ArithmeticOperator::Divide => "/",
            },
        )
    }
}

pub struct NumericMutator;

impl VisitorMut for NumericMutator {
    fn replace_expression(
        &mut self,
        node: &full_moon::ast::Expression,
    ) -> Option<full_moon::ast::Expression> {
        if let full_moon::ast::Expression::Number(num) = node {
            let token_str = num.token().to_string();
            let number: f64 = if let Some(hex) = token_str
                .strip_prefix("0x")
                .or_else(|| token_str.strip_prefix("0X"))
            {
                u64::from_str_radix(hex, 16)
                    .map(|n| n as f64)
                    .unwrap_or_else(|_| token_str.parse().unwrap())
            } else {
                token_str.parse().unwrap()
            };

            // let second_num: f64 = rng().random::<f64>() * 50.0;
            let second_num: f64 = rng().random_range(1..50).try_into().unwrap();
            let op = ArithmeticOperator::random();

            let first_num = op.invert().apply(number, second_num);

            let expr = utils::extract_expression(&format!("({} {} {})", first_num, op, second_num))
                .expect("Failed to extract numeric expression");

            return Some(expr);
        }

        None
    }
}

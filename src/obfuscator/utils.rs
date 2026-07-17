#[macro_export]
macro_rules! randomizable_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            pub fn random() -> Self {
                let mut rng = rand::rng();

                let variants = [$( $name::$variant ),*];
                *variants.choose(&mut rng).unwrap()
            }
        }
    };
}

pub fn index_of<T>(list: &[T], value: T) -> usize
where
    T: PartialEq<T>,
{
    list.iter().position(|v| *v == value).unwrap()
}

pub fn extract_expression(code: &String) -> Result<full_moon::ast::Expression, String> {
    let return_code = format!("return {code}");

    let encrypted_ast =
        full_moon::parse(&return_code).map_err(|e| format!("Failed to parse ast: {:?}", e))?;

    let last_stmt = encrypted_ast
        .nodes()
        .last_stmt()
        .ok_or(String::from("Failed to get last statement"))?;

    if let full_moon::ast::LastStmt::Return(ret_stmt) = last_stmt {
        let expr = ret_stmt
            .returns()
            .iter()
            .next()
            .expect("Failed to get return expr");

        Ok(expr.clone())
    } else {
        Err(String::from("Failed to find return statement"))
    }
}

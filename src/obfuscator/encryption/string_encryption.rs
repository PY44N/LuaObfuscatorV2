use full_moon::{
    ast::{self, LastStmt},
    parse,
    visitors::VisitorMut,
};
use rand::{Rng, distr::Alphanumeric, random_range};

pub struct StringEncryptor;

fn cut_first_and_last(s: &str, n: usize) -> &str {
    // 1. Find the byte index where the first n characters end
    let start = s
        .char_indices()
        .nth(n)
        .map(|(idx, _)| idx)
        .unwrap_or(s.len());

    // 2. Find the byte index where the last n characters start
    let end = s
        .char_indices()
        .nth_back(n - 1)
        .map(|(idx, _)| idx)
        .unwrap_or(0);

    // 3. Ensure indices are valid and return the subslice
    if start < end { &s[start..end] } else { "" }
}

fn xor_multibyte_key(bytes: &[u8], key: &[u8]) -> Vec<u8> {
    let mut new_bytes = Vec::new();
    for i in 0..bytes.len() {
        new_bytes.push(bytes[i] ^ key[i % key.len()]);
    }

    new_bytes
}

fn generate_decryption_code(byte_string: String, key: String) -> String {
    format!(
        "return (function(str)
    local function bxor(a, b)
        local result = 0
        local bitval = 1

        while a > 0 or b > 0 do
            local a_bit = a % 2
            local b_bit = b % 2

            if a_bit ~= b_bit then
                result = result + bitval
            end

            a = math.floor(a / 2)
            b = math.floor(b / 2)
            bitval = bitval * 2
        end

        return result
    end

    key = '{}'
    local out = ''
    for i = 0, #str - 1 do
        local key_i = i % #key + 1
        --   print(key_i)
        local a = string.byte(string.sub(str, i + 1, i + 1))
        local b = string.byte(string.sub(key, key_i, key_i))
        -- print(a)
        -- print(b)
        out = out .. string.char(bxor(a, b))
    end

    return out

end)('{}')",
        key, byte_string
    )
}

fn encrypt_string(string_value: &str) -> String {
    let key: String = rand::rng()
        .sample_iter(&Alphanumeric)
        .take(random_range(16..=32)) // Number of characters
        .map(char::from)
        .collect();

    let encrypted_bytes = xor_multibyte_key(string_value.as_bytes(), key.as_bytes());
    let encrypted_byte_string: String = encrypted_bytes
        .iter()
        .map(|b| "\\".to_owned() + &b.to_string())
        .collect();

    generate_decryption_code(encrypted_byte_string, key)
}

impl VisitorMut for StringEncryptor {
    fn fold_expression(
        &mut self,
        expr: &full_moon::ast::Expression,
    ) -> Option<full_moon::ast::Expression> {
        match expr {
            ast::Expression::String(token_ref) => {
                let string = token_ref.token().to_string();
                let string_value = if let Some(c) = string.chars().next()
                    && c == '['
                {
                    cut_first_and_last(&string, 2)
                } else {
                    cut_first_and_last(&string, 1)
                };

                // NOTE: This is a really hacky way of doing this
                // TODO: Clean this up/find a better approach

                let encrypted_ast = parse(&encrypt_string(string_value))
                    .expect("Failed to parse generated string encryption code");

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

                panic!("Failed to find return statment")
            }
            other => None,
        }
    }
}

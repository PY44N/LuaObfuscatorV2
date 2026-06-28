use full_moon::visitors::VisitorMut;

use crate::obfuscator::encryption::string_encryption::StringEncryptor;

pub fn encrypt(input: &mut String) -> String {
    let ast = full_moon::parse(input).expect("Failed to parse input code");

    let mut string_encryptor = StringEncryptor;

    let string_encrypted_ast = string_encryptor.visit_ast(ast);

    string_encrypted_ast.to_string()
}

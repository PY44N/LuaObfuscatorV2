use full_moon::visitors::VisitorMut;

use crate::{
    obfuscation_settings::ObfuscationSettings,
    obfuscator::encryption::string_encryption::StringEncryptor,
};

pub fn encrypt(input: &String, settings: &ObfuscationSettings) -> String {
    let ast = full_moon::parse(input).expect("Failed to parse input code");

    if settings.encrypt_strings {
        let mut string_encryptor = StringEncryptor;

        let string_encrypted_ast = string_encryptor.visit_ast(ast);

        return string_encrypted_ast.to_string();
    }

    return input.clone();
}

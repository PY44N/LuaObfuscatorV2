use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    process::Command,
};

use lua_deserializer::deserializer::Deserializer;
use lua_minifier::minifier::minify;
use obfuscator::vm_generator::VMGenerator;

use crate::{
    obfuscation_settings::ObfuscationSettings, obfuscator::encryption::constant_encryption,
};

pub mod obfuscation_settings;
pub mod obfuscator;

static FINAL_FILE: &str = "temp4.lua";

fn main() {
    let settings = ObfuscationSettings::new();

    if Path::new("temp").is_dir() {
        fs::remove_dir_all("temp").unwrap();
    }
    fs::create_dir("temp").unwrap();

    fs::copy("Input.lua", "temp/temp1.lua").unwrap();

    let luac_command = "luac";

    println!("[Obfuscator] Encrypting Constants...");

    let mut initial_code =
        fs::read_to_string("temp/temp1.lua").expect("Failed to read file temp1.lua");
    constant_encryption::encrypt(&mut initial_code);

    fs::write("temp/temp2.lua", initial_code).expect("Failed to write to file temp2.lua");

    println!("[Obfuscator] Compiling...");

    Command::new(luac_command)
        .arg("temp2.lua")
        .current_dir("temp")
        .output()
        .expect("Failed to compile lua binary");

    println!("[Obfuscator] Reading file...");

    let mut reader = BufReader::new(File::open("temp/luac.out").unwrap());
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    println!("[Obfuscator] Deserializing...");

    let mut deserializer = Deserializer::new(buffer);
    let main_chunk = deserializer.deserialize();

    println!("[Obfuscator] Generating VM...");

    let vm_generator = VMGenerator::new();
    let vm = vm_generator.generate(main_chunk, settings);

    fs::write("temp/temp3.lua", vm).expect("Failed to write vm to file");

    println!("[Obfuscator] Minifying...");

    // Command::new("node")
    //     .arg(".")
    //     .current_dir("minifier")
    //     .output()
    //     .expect("Failed to minify");
    let minified = minify("temp/temp3.lua").expect("Failed to minify");
    fs::write("temp/temp4.lua", minified).expect("Failed to write minified to file");

    println!("[Obfuscator] Running...");

    let output = Command::new("lua")
        .arg(FINAL_FILE)
        .current_dir("temp")
        .output()
        .expect(&format!("Failed to run {}", FINAL_FILE));

    let output_string: String = output.stdout.into_iter().map(|v| v as char).collect();
    let output_error: String = output.stderr.into_iter().map(|v| v as char).collect();

    println!("Program output:\n{}", output_string);
    if output_error != "" {
        println!("Program Error:\n{}", output_error);
    }
}

use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    process::Command,
};

use obfuscator::vm_generator::VMGenerator;
use util::read_stream::ReadStream;

use crate::{bytecode::deserializer::Deserializer, obfuscation_settings::ObfuscationSettings};

pub mod bytecode;
pub mod obfuscation_settings;
pub mod obfuscator;
pub mod util;

fn main() {
    let settings = ObfuscationSettings::new();

    if Path::new("temp").is_dir() {
        fs::remove_dir_all("temp").unwrap();
    }
    fs::create_dir("temp").unwrap();

    fs::copy("Input.lua", "temp/temp1.lua").unwrap();

    let luac_command = if cfg!(target_os = "windows") {
        "luac"
    } else {
        "luac5.1"
    };

    println!("[Obfuscator] Compiling...");

    Command::new(luac_command)
        .arg("temp1.lua")
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
    let vm = vm_generator.generate(main_chunk, &settings);

    fs::write("temp/temp2.lua", vm).expect("Failed to write vm to file");

    println!("[Obfuscator] Minifying...");

    Command::new("node .")
        .current_dir("minifier")
        .output()
        .expect("Failed to minify");

    println!("[Obfuscator] Running...");

    let output = Command::new(if cfg!(target_os = "windows") {
        "lua"
    } else {
        "lua5.1"
    })
    .arg("temp2.lua")
    .current_dir("temp")
    .output()
    .expect("Failed to run temp2.lua");

    let output_string: String = output.stdout.into_iter().map(|v| v as char).collect();
    let output_error: String = output.stderr.into_iter().map(|v| v as char).collect();

    println!("Program output:\n{}", output_string);
    if output_error != "" {
        println!("Program Error:\n{}", output_error);
    }
}

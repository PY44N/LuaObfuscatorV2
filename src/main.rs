use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    process::Command,
};

use clap::Parser;
use lua_deserializer::deserializer::Deserializer;
use obfuscator::vm_generator::VMGenerator;

use crate::{
    obfuscation_settings::ObfuscationSettings,
    obfuscator::{encryption::constant_encryption, mutations, uglification::inliner::Inliner},
};

pub mod obfuscation_settings;
pub mod obfuscator;

static FINAL_FILE: &str = "temp5.lua";

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the file to obfuscate
    #[arg(short, long)]
    file: String,

    /// Run the program after obfuscated
    #[arg(short, long, default_value_t = false)]
    run: bool,
}

fn main() {
    let settings = ObfuscationSettings::new();

    let args = Args::parse();

    if Path::new("temp").is_dir() {
        fs::remove_dir_all("temp").unwrap();
    }
    fs::create_dir("temp").unwrap();

    if !Path::new(&args.file).exists() {
        println!("File {} does not exist", args.file);
        return;
    }

    fs::copy(&args.file, "temp/temp1.lua").unwrap();

    let luac_command = "luac";

    println!("[Obfuscator] Encrypting Constants...");

    let initial_code = fs::read_to_string("temp/temp1.lua").expect("Failed to read file temp1.lua");
    let encrypted_code = constant_encryption::encrypt(&initial_code, &settings);
    let mutated_code = mutations::apply(&encrypted_code, &settings);

    fs::write("temp/temp2.lua", mutated_code).expect("Failed to write to file temp2.lua");

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
    let vm = vm_generator.generate(main_chunk, &settings);

    fs::write("temp/temp3.lua", vm.clone()).expect("Failed to write vm to file");

    let inlined_code = Inliner::inline_from_code(vm);
    let mutated_inlined_code = mutations::apply(&inlined_code, &settings);

    fs::write("temp/temp4.lua", mutated_inlined_code)
        .expect("Failed to write inlined code to file");

    println!("[Obfuscator] Minifying...");

    Command::new("node")
        .arg(".")
        .current_dir("minifier")
        .output()
        .expect("Failed to minify");

    if args.run {
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

    fs::copy("temp/".to_owned() + FINAL_FILE, "Out.lua").expect("Failed to copy final file");

    if cfg!(not(debug_assertions)) {
        fs::remove_dir_all("temp").expect("Failed to delete temp directory");
    }
    println!("[Obfuscator] Your obfuscated program has been written to Out.lua");
}

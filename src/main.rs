use std::{
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    process::Command,
};

use util::memory_stream::MemoryStream;

use crate::bytecode::deserializer::Deserializer;

pub mod bytecode;
pub mod util;

fn main() {
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

    Command::new(luac_command)
        .arg("temp1.lua")
        .current_dir("temp")
        .output()
        .expect("Failed to compile lua binary");

    let mut reader = BufReader::new(File::open("temp/luac.out").unwrap());
    let mut buffer = Vec::new();

    reader.read_to_end(&mut buffer).unwrap();

    let mut deserializer = Deserializer::new(buffer);
    let main_chunk = deserializer.deserialize();
}

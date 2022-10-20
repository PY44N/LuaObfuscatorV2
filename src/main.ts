import { readFileSync, mkdirSync, copyFileSync, existsSync, rmSync } from "fs";
import { execSync } from "child_process";
import { Deserializer } from "./bytecode/Deserializer";

// if (existsSync("temp")) rmSync("temp", { recursive: true });
// mkdirSync("temp");

// copyFileSync("Input.lua", "./temp/temp1.lua");

// //TODO: Maybe strip comments and minify before compiling
// const luac = process.platform == "linux" ? "luac5.1" : "luac";
// execSync(`${luac} temp1.lua`, { cwd: "temp" });

let compiled = readFileSync("./temp/luac.out", "binary")
  .split("")
  .map((v) => v.charCodeAt(0));

let deserializer = new Deserializer(compiled);
deserializer.deserialize();

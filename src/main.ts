import {
  readFileSync,
  mkdirSync,
  copyFileSync,
  existsSync,
  rmdirSync,
} from "fs";
import { execSync } from "child_process";
import { Deserializer } from "./bytecode/Deserializer";

// if (existsSync("temp")) rmdirSync("temp", { recursive: true });
// mkdirSync("temp");

// copyFileSync("Input.lua", "./temp/temp1.lua");

// execSync("luac temp1.lua", { cwd: "temp" });

let compiled = readFileSync("./temp/luac.out", "binary")
  .split("")
  .map((v) => v.charCodeAt(0));

console.log(compiled);

let deserializer = new Deserializer(compiled);

import {
  readFileSync,
  mkdirSync,
  copyFileSync,
  existsSync,
  rmdirSync,
} from "fs";
import { execSync } from "child_process";

if (existsSync("temp")) rmdirSync("temp", { recursive: true });
mkdirSync("temp");

copyFileSync("Input.lua", "./temp/temp1.lua");

execSync("luac temp1.lua", { cwd: "temp" });

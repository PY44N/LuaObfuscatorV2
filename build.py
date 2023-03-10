import subprocess
import sys
import shutil
import os

try:
    subprocess.run(["node", "-v"])
except:
    print("Nodejs not found please install from https://nodejs.org/en/download/")
    exit()

try:
    subprocess.run(["npm", "-v"])
except:
    print("NPM not found please install from https://docs.npmjs.com/downloading-and-installing-node-js-and-npm")
    exit()

try:
    subprocess.run(["cargo", "-V"])
except:
    print("Nodejs not found please install from https://www.rust-lang.org/tools/install")
    exit()

os_target_map = {
    "win32": "node18-win-x64",
    "darwin": "node18-macos-x64",
    "linux": "node18-linux-x64",
}

subprocess.run(["npm", "i"], cwd="minifier")
subprocess.run(["npx", "pkg", "-t", os_target_map[sys.platform], "main.js"], cwd="minifier")

if os.path.isdir("build"):
    shutil.rmtree("build")
os.mkdir("build")

input = "minifier/main"
output = "build/minifier"

if sys.platform == "win32":
    input += ".exe"
    output += ".exe"

subprocess.run(["cp", input, output])

subprocess.run(["cargo", "build", "--release"])

input = "target/release/lua_obfuscator"
output = "build/obfuscator"

if sys.platform == "win32":
    input += ".exe"
    output += ".exe"

subprocess.run(["cp", input, output])


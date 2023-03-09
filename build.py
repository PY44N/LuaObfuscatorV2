import os
import argparse

parser = argparse.ArgumentParser(description='A Lua Obfuscator')
parser.add_argument("-d", "--debug", action="store_true", help="Compile the program in debug mode")
parser.add_argument("-r", "--run", action="store_true", help="Run the program after compiling")

args = parser.parse_args()

try:
    os.system("fer -v")
except:
    print("Nodejs not found please install from https://nodejs.org/en/download/")
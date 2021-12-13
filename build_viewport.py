#!/bin/python3

import os
import shutil
import sys
import re

release_arg = ""
from_dir = "debug"

if len(sys.argv) == 2 and sys.argv[1] == "release":
    release_arg = "--release"
    from_dir = "release"

# Remove previous artifacts
if os.path.isdir("frontend/src/wasm"):
    shutil.rmtree("frontend/src/wasm")

if os.path.isfile("frontend/public/viewport_bg.wasm"):
    os.remove("frontend/public/viewport_bg.wasm")

# Prepare temporary directory
os.mkdir("temp")

# Actual build process
os.system(f"cargo build {release_arg} --manifest-path viewport/Cargo.toml --target wasm32-unknown-unknown")
os.system(f"wasm-bindgen --target web --out-dir temp viewport/target/wasm32-unknown-unknown/{from_dir}/viewport.wasm")

# Patch viewport.js

script = ""
with open("temp/viewport.js", "r") as file:
    script = file.read()

script = re.sub(r'input = new URL\(.*, import\.meta\.url\);', "", script);
script = script.replace("var ret = globalThis.globalThis", "var ret = window.globalThis.globalThis")
script = script.replace("var ret = self.self", "var ret = window.self.self")

with open("temp/viewport.js", "w") as file:
  file.write(script)

# Copy necessary files to their appropriate positions
os.mkdir("frontend/src/wasm")
shutil.copyfile("temp/viewport_bg.wasm", "frontend/public/viewport_bg.wasm")
shutil.copyfile("temp/viewport_bg.wasm.d.ts", "frontend/src/wasm/viewport_bg.wasm.d.ts")
shutil.copyfile("temp/viewport.d.ts", "frontend/src/wasm/viewport.d.ts")
shutil.copyfile("temp/viewport.js", "frontend/src/wasm/viewport.js")

# Cleanup
shutil.rmtree("temp")
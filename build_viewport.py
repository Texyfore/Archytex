#!/bin/python3

import os
import shutil
import sys

release_arg = ""
from_dir = "debug"

if len(sys.argv) == 2 and sys.argv[1] == "release":
    release_arg = "--release"
    from_dir = "release"

# Prepare temporary directory
os.mkdir("temp")

# Actual build process
os.system(f"cargo build {release_arg} --manifest-path viewport/Cargo.toml --target wasm32-unknown-unknown")
os.system(f"wasm-bindgen --target web --out-dir temp viewport/target/wasm32-unknown-unknown/{from_dir}/viewport.wasm")

# Copy necessary files to their appropriate positions
os.mkdir("frontend/src/wasm")
shutil.copyfile("temp/viewport_bg.wasm", "frontend/public/viewport_bg.wasm")
shutil.copyfile("temp/viewport_bg.wasm.d.ts", "frontend/src/wasm/viewport_bg.wasm.d.ts")
shutil.copyfile("temp/viewport.d.ts", "frontend/src/wasm/viewport.d.ts")
shutil.copyfile("temp/viewport.js", "frontend/src/wasm/viewport.js")

# Cleanup
shutil.rmtree("temp")

# Patch scripts

# TODO
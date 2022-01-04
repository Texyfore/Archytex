#!/usr/bin/python3

import os;
os.chdir("viewport")
os.system("wasm-pack build --dev")
os.chdir("../frontend")
os.system("yarn upgrade viewport")
os.chdir("..")
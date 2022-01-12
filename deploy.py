#!/usr/bin/python3

import os
import sys

def main():
    profile = "--dev"
    if len(sys.argv) == 2 and sys.argv[1] == "--release":
        profile = "--release"

    os.chdir("viewport")
    os.system(f"wasm-pack build {profile}")
    os.chdir("../frontend")
    os.system("yarn upgrade viewport")
    os.system("yarn start")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        os.chdir("..")
        sys.exit()
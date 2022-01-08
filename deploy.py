#!/usr/bin/python3

import os
import sys

def main():
    os.chdir("viewport")
    os.system("wasm-pack build --dev")
    os.chdir("../frontend")
    os.system("yarn upgrade viewport")
    os.system("yarn start")

if __name__ == "__main__":
    try:
        main()
    except KeyboardInterrupt:
        os.chdir("..")
        sys.exit()
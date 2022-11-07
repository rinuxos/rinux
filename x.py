import sys;
import os;

def build(typ):
    os.system("epearl --gen --nocolor --quiet")
    if typ == True:
        os.system("epearl release")
        os.system("cargo build --release")
        os.system("cargo bootimage --release")
    else:
        os.system("epearl build")
        os.system("cargo build")
        os.system("cargo bootimage")

def help():
    print("Available commands:\n     - release\n     - build\n     - run")

if __name__ == "__main__":
    if len(sys.argv) < 2:
        help()
        sys.exit(0)
    cmd = str(sys.argv[1])
    if cmd == "build":
        build(False)
    elif cmd == "run":
        os.system("cargo run")
    elif cmd == "release":
        build(True)
    else:
        help()
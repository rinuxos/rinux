import enderpearl
import shutil
import sys
import os


def build(release: bool):
    enderpearl.run(["--gen","--nocolor","--quiet"])
    if release == True:
        os.system("cargo clean")
        enderpearl.run(["release"])
        os.system("cargo build --release")
        os.system("cargo bootimage --release")
    else:
        enderpearl.run(["build"])
        os.system("cargo build")
        os.system("cargo bootimage")
    

def _help():
    print("Available commands:\n     - build: Builds in debug mode, pass '--release' to build in release\n     - run: Builds and runs, pass '--release' to run in release\n     - init: Initializes the project\n     - gen: Generates the project config files\n     - stasis: Installes dependencies from stasis")


def clean(full:bool=False):
    if full == True:
        os.system("cargo clean")
    dir_path = "./enderpearl/__pycache__"
    try:
        shutil.rmtree(dir_path)
    except OSError as e:
       pass



def main():
    __release = False

    if '--release' in sys.argv:
        __release = True
    if '--clean' in sys.argv:
        clean()
        return

    if len(sys.argv) < 2:
        _help()
        sys.exit(0)
    cmd = str(sys.argv[1])

    if cmd == "build":
        build(__release)
        return
    elif cmd == "run":
        if __release:
            os.system("cargo run --release")
        else:
            os.system("cargo run")
        return
    elif cmd == "init":
        os.system("cargo install bootimage")
        enderpearl.run(["--mkdir","--init","--gen",])
        enderpearl.run(["--stasis"])
        return
    elif cmd == "gen":
        enderpearl.run(["--gen"])
        return
    elif cmd == "stasis":
        enderpearl.run(["--stasis"])
        return
    else:
        _help()

if __name__ == "__main__":
    clean()
    main()
    clean()
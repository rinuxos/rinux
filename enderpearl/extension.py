import enderpearl.extensioncore as extensioncore
import enderpearl
import os


RELEASE: bool


def helper() -> str: return extensioncore.hlp()

def command_parser(cmd: str) -> None:
    if cmd.startswith("help"): return print(helper())

    elif cmd.startswith("build"):
        build(RELEASE)
        return
    elif cmd.startswith("run"):
        if RELEASE:
            os.system("cargo run --release")
        else:
            os.system("cargo run")
        return
    elif cmd.startswith("init"):
        os.system("cargo install bootimage")
        extensioncore.run("--mkdir --init --gen --stasis")
        return
    elif cmd.startswith("gen"):
        extensioncore.run("--gen")
        return
    elif cmd.startswith("stasis"):
        extensioncore.run("--stasis")
        return
    elif cmd.startswith("help"):
        print(helper())
        return



    else: return enderpearl.parser.default_run(cmd)



def build(release: bool) -> None:
    extensioncore.run("--gen --nocolor --quiet")
    if release == True:
        extensioncore.run("release")
    else:
        extensioncore.run("build")
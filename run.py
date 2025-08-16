import subprocess
import argparse
import os

def build():
    subprocess.run(["cargo", "build", "--quiet"])

# LLVM_SYS_170_PREFIX=$(brew --prefix llvm) cargo build

def run(file_name):
    subprocess.run(["cargo", "run", "--quiet", "--", "--file", file_name])

# TODO: add file name corresponding to desired .ll output file name...
def convert_ll_to_executalble():
    subprocess.run(["clang", "lcc_module.ll", "-o", "lcc_module"])

def execute():
    subprocess.run("./lcc_module")

def main():
    # LLVM_SYS_200_PREFIX="$(brew --prefix llvm@20)"
    llvm_prefix = subprocess.check_output(["brew", "--prefix", "llvm@17"]).decode().strip()
    os.environ["LLVM_SYS_170_PREFIX"] = llvm_prefix

    cmd_args = parse_cmd_line_args()
    build()
    run(cmd_args.file)

    convert_ll_to_executalble()
    #execute()



def parse_cmd_line_args():
    parser = argparse.ArgumentParser()
    parser.add_argument("--file", type=str, required=True)
    cmd_line_args = parser.parse_args()
    return cmd_line_args

main()



#cargo build --quiet
#cargo run  --quiet

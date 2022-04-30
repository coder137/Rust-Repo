import subprocess
import os


def run_command(command: str, pout=False):
    output = subprocess.run(command, capture_output=True)
    odata = output.stdout.decode("utf-8").strip()
    if pout:
        print(output)
        print(odata)
    return odata


# MAIN

# 1. Build the target with -C instrument-coverage
os.environ['RUSTFLAGS'] = '-C instrument-coverage'
run_command("cargo build")
# print(f"RUSTFLAGS: {os.getenv('RUSTFLAGS')}")

# 2. Run the built target
run_command("target/debug/code_coverage.exe")
rustc_sysroot = run_command("rustc --print sysroot")

# 3. Merge to profdata
relative_llvm_path = "lib/rustlib/x86_64-pc-windows-msvc/bin"
command = f"{rustc_sysroot}/{relative_llvm_path}/llvm-profdata.exe merge -sparse default.profraw -o default.profdata"
run_command(command)

# 4. Use profdata to get html output
command = f"{rustc_sysroot}/{relative_llvm_path}/llvm-cov.exe show -Xdemangler=rustfilt target/debug/code_coverage.exe -instr-profile=default.profdata -show-line-counts-or-regions -show-instantiations -format=html"
html = run_command(command)
# print(html)

# 5. Write index.html file
f = open("index.html", "w")
f.write(html)
f.close()

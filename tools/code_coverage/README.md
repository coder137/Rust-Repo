# Code Coverage

## Pre-requisites

- Requires a minimum of Rust 1.60.0
- Read [blog post here](https://blog.rust-lang.org/2022/04/07/Rust-1.60.0.html)


## Installation

- cargo install rustfilt
- rustup component add llvm-tools-preview

# Run the example

- `python3 run.py`
  - NOTE: This example has been adapted for target `x86_64-pc-windows-msvc`.
  - Update the script if required variable `relative_llvm_path` with your target
  - Check your target through `rustup default`
- This will create `default.profraw` (initial capture)
- `default.profraw` is converted to `default.profdata`
- `default.profdata` is converted to `index.html` file

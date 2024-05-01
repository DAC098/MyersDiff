# Setup

the code is written in Rust and only uses a single package for handling command
line arguments. you can install the latest version of Rust by following the
[Getting Started](https://www.rust-lang.org/learn/get-started) posted on their
site.

in the root of the project run `cargo build` for debug or `cargo build
--release` for an optimized binary file. the command will download the required
packages to build the program and create the executable. you can find the build
binaries in the `target` directory under `debug` or `release`.

if you want to build and run the program then you can run `cargo run -- help`
or `cargo run --release -- help` and both will display the help output of the
program.

`help` command output:

```
Usage: project.exe [OPTIONS] <COMMAND>

Commands:
  base        performs the base form of the algorithm and returns the
                  resulting edit distance
  modified    similar to base but with a modified way of storing the data
                  same output
  printed     prints out each step the algorithm takes when performing
                  calculations
  traced      creates a trace of each depth calculated, used for getting a
                  backtrace of the operations needed to convert string b to a
  operations  lists the operations needed to convert string b to a
  help        Print this message or the help of the given subcommand(s)

Options:
  -f, --from <FROM>  the string you want to convert from [default: abcabba]
  -t, --to <TO>      the string you want to convert to [default: cbabac]
  -h, --help         Print help
```

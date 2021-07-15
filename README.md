# Riff
file comparison utility built in rust

## Install

to install the binary:
```
cargo install riff-comp
```

## Usage

Riff provides a simple cli to work with:
```
$ riff old_file new_file
```
the command acts like the UNIX ```diff``` utility and prints the differences to the output; with the changed lines
being marked differently from the lines common between the two files.

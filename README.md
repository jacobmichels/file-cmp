# file-cmp
Super simple command line utility for inspecting text files. 

## Description
This tool takes in files to compare and outputs if they contain the same lines the same number of times (even if the lines are not in the same order).

This is useful for comparing program logs/output that always contain the same contents but aren't always printed in the same order. 

## Example usage

### Situation 1
Let's say we have the following files, `file1.txt` and `file2.txt`.

file1.txt
```
foo
bar
baz
```

file2.txt
```
bar
foo
baz
```

`file-cmp file1.txt file2.txt` outputs `Equal` and exitcode 0.

### Situation 2
Let's consider the same two files again.

file1.txt
```
foo
baz
```

file2.txt
```
bar
foo
```

`file-cmp file1.txt file2.txt` outputs `Not equal` and exitcode 1.

## Compiling
This app requires rust nightly. The following instructions assume you have git and rustup installed.

1. `git clone` the repo and cd into the created directory
2. Run `rustup override set nightly` to use the nightly compiler for this directory
3. `cargo build`


### Tests
Run tests with `cargo test`

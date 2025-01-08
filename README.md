# transpose-cmd
A command-line tool to output the transpose of a CSV file.

# Example Usage
```
$ cat matrix.csv 
1, 2, 3
4, 5, 6

$ cargo run matrix.csv
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/transpose matrix.csv`
1,4
2,5
3,6
```

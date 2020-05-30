# Code examples

- read lines from file

```rust
if let Ok(lines) = read_lines(value) {
    for line in lines {
        if let Ok(word) = line {
            //println!("{}", word);
            let v: Vec<&str> = word.splitn(2, '#').collect();
            println!("{}", v[0]);
        }
    }
}
```
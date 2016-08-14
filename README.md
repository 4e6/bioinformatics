# Bioinformatics

Algorighms for [Coursera Bioinformatics][coursera-bioinformatics]
course. Project divided into library `src` and executable programming
assignments `examples`

### Examples

Contains programming assignments for the course. Each assignment implemented as
an executable, that read input data from `stdin` and write result to `stdout`.

Run executable by name:

``` shell
cargo run --release --executable <name>
```

For every programming assignment `data` directory contains input datasets as
well as the expected output. `verify` script runs each assignment on
corresponding dataset and compares result with expected output:

``` shell
examples/verify
...
    Finished release [optimized] target(s) in 0.24 secs
     Running `target/release/examples/reverse_complement`
OK reverse_complement dataset_3_2
```

### Unit tests

Run unit tests:

``` shell
cargo test
```

[coursera-bioinformatics]: https://www.coursera.org/specializations/bioinformatics

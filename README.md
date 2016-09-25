# Bioinformatics

Algorithms for [Coursera Bioinformatics][coursera-bioinformatics]
course. Project divided into library `src` and executable programming
assignments `examples`.

### Examples

Directory with executable programming assignments for the course. For every
programming task, `data` directory contains input datasets, as well as the
expected output. `verify` script runs each assignment on the corresponding
dataset and compares the result with expected output.

``` shell
examples/verify
...
    Finished release [optimized] target(s) in 0.24 secs
     Running `target/release/examples/reverse_complement`
OK reverse_complement dataset_3_2
```

Run single executable by name:

``` shell
cargo run --release --executable <name>
```

### Unit tests

Run unit tests:

``` shell
cargo test
```

[coursera-bioinformatics]: https://www.coursera.org/specializations/bioinformatics

# Bioinformatics

Programming assignments for the
[Coursera Bioinformatics][coursera-bioinformatics] course.

## Build

Build executables for programming assignments:

``` shell
cargo build [--release]
```

### Unit tests

Run cargo tests:

``` shell
cargo test
```

### Integration tests

Compare output of each executable programming assignment with saved answer:

``` shell
tests/verify
```

[coursera-bioinformatics]: https://www.coursera.org/specializations/bioinformatics
abs
===

Various (incorrect) implementations of abs function. I use these
variants to introduce random testing and the problem of integer
overflows.

This crate two correct implementations (abs and sat_abs)
that should demonstrate ways how one can address integer overflows.


### Installation

To add sort to your crate, just add the following
dependency.

```toml
[dependencies.abs]
git = "https://github.com/christoffetzer/abs"
```

The `abs` crate depends on two external crates:

- rndtester: https://github.com/christoffetzer/rndtester

	a very simple crate to help with writing
	random test cases. This only demonstrate how random
	test cases could be programed manually. 

- quickcheck: https://github.com/BurntSushi/quickcheck

### Test

Test the code by pulling it with cargo or git and execute:

```sh
cargo test
```

### Documentation

Build the documentation by running

```sh
cargo doc
```

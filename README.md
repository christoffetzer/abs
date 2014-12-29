abs
===

This crate contains three incorrect implementations of the 
'abs' function. I use these variants to introduce unit and random
testing. I chose 'abs' since it looks so trival but it contains 
an integer overflow.

This crate also contains two (hopefully) correct implementations of
'abs': abs and sat_abs. They should demonstrate ways on how one
could address the integer overflow in the 'abs' function.


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

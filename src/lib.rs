#![experimental]

#![feature(phase)]
#[phase(plugin)]
extern crate quickcheck_macros;
extern crate quickcheck;

extern crate core;

#[doc = "
An incorrect implementation of abs function. 

Our intention is to measure how quickly we can find
this bug with random testing.

Before we do so, we define a unit test for 'abs1'. Any unit test
will most likely find this bug.

```
#[test]
fn test_abs1() {
	assert!(abs::abs1(1) == 1);
	assert!(abs::abs1(-1) == 1);
	assert!(abs::abs1(-2) == 2);
}
```

We also provide a simple random unit tester for
the 'abs1' function.

```
extern crate rndtester;
extern crate abs;

use rndtester::rnd_values;

#[test]
fn rnd_abs1() {
    let (mut rit, mut v) = rndtester::rnd_values();
    for i in rit {
		assert!(abs::abs1(i) >= 0);
	}
}
```
"]

pub fn abs1(r: int) -> int {
	if r>0 {
		r
	} else {
		r 
	}
} 

#[doc = "
Our first attempt to correct the abs function. 

This implementation still contains a bug. 
Our intention is to measure if and how quickly we can find
this bug with random testing.

Before we do so, we define a unit test for 'abs2'. It
is not clear that a unit test will find this bug.

```test_harness
#[test]
fn test_abs2() {
	assert!(abs::abs2(1) == 1);
	assert!(abs::abs2(-1) == 1);
	assert!(abs::abs2(-2) == 2);
}
```

Random tester for 'abs2' will not find the bug either: the likelihood
to detect the problem within a reasonable amount of time is very small
is very small.

```test_harness
extern crate rndtester;
extern crate abs;

use rndtester::rnd_values;

#[test]
fn rnd_abs2() {
    let (mut rit, mut v) = rndtester::rnd_values();
    for i in rit {
		assert!(abs::abs2(i) >= 0);
	}
}
```
"]
pub fn abs2(r: int) -> int {
	if r>0 {
		r
	} else {
		-r 
	}
} 

#[doc = "
Our second attempt to correct the abs function. 

This implementation still contains a bug. 

Our intention is still to measure if and how quickly we can find
this bug with random testing.

Before we do so, we define a unit test for 'abs3'. It
is not clear that a unit test will find this bug.

```test_harness
#[test]
fn test_abs3() {
	assert!(abs::abs3(1i) == 1);
	assert!(abs::abs3(-1i32) == 1);
	assert!(abs::abs3(-2i8) == 2);
}
```

Random tester for 'abs3' restricts the value range to i8 to
increase our changes to find the bug. 

```test_harness
extern crate rndtester;
extern crate abs;

use rndtester::rnd_values;
use rndtester::RndTester;

#[allow(unused_comparisons)]
#[test]
fn rnd_abs3() {
    let (mut rit, mut v) = rndtester::rnd_values();
    for i in range(0u, 20) {
		let vv : i8 = v.rnd_value();
		assert!(abs::abs3(vv) >= 0);
	}
}
```
"]


pub fn abs3<T: std::num::Num + std::cmp::PartialOrd>(r: T) -> T {
	if r >= std::num::Zero::zero() {
		r
	} else {
		-r 
	}
} 

#[doc = "
Our last attempt to correct the abs function. 

This implementation should be correct: we fix the return type to
ensure that we can let the caller know that an overflow has happened.
An alternative would be to return an unsigned integer instead.

```test_harness
#[test]
fn test_abs() {
	assert!(abs::abs(1i) == Some(1));
	assert!(abs::abs(-1i32) == Some(1));
	assert!(abs::abs(-2i8) == Some(2));
	assert!(abs::abs(-128i8) == None);
}
```

Random tester for 'abs'  - this should not find a bug.

```test_harness
extern crate rndtester;
extern crate abs;

use rndtester::rnd_values;
use rndtester::RndTester;

#[allow(unused_comparisons)]
#[test]
fn rnd_abs() {
    let (mut rit, mut v) = rndtester::rnd_values();
    for i in range(0u, 2000) {
		let vv : i8 = v.rnd_value();
		match abs::abs(vv) {
			Some(val) =>   assert!(val >= 0),
			None =>  assert!(true),
		}
	}
}
```
"]


pub fn abs<T: std::num::Num + std::num::Int + std::cmp::PartialOrd>(r: T) -> Option<T> {
	if r >= std::num::Zero::zero() {
		Some(r)
	} else if r == std::num::Int::min_value() {
		None
	} else {
		Some(-r)
	}
} 

#[quickcheck]
fn check_abs(xs: i8) -> bool {
	match abs(xs) {
		Some(val) =>   if xs > 0 { xs == val }
					   else { val >= 0 && -xs == val },
		None =>  xs == -128,
	}
}


#[doc = "
abs for saturation integers.

The idea of saturation integers is that if a result
of an operation is out of bounds for a given data type,
one uses the MIN or MAX value:

MAX < result  => result = MAX

MIN <= result <= MAX => result

result < MIN  => result = MIN

Saturation integers are for example ideal in the context range 
iterators: if we subtract or add a number, they should not wrap around. 

Here is the unit test for sat_abs for i8: 

```test_harness

#[test]
fn test_sat_abs() {
	assert!(abs::sat_abs(0i8) == 0);
	assert!(abs::sat_abs(-127i8) == 127i8);
	assert!(abs::sat_abs(-128i8) == 127i8);
}
```
"]


pub fn sat_abs(r: i8) -> i8 {
	if r>0 {
		r
	} else {
		let (v,overflow) = unsafe { core::intrinsics::i8_sub_with_overflow(0i8,r) };
		if overflow {
			std::num::Int::max_value()
		} else {
			v
		}
	}
} 


#[quickcheck]
fn check_sat_abs(xs: i8) -> bool {
	let val = sat_abs(xs);
	
	if xs > 0 { xs == val }
	else { val >= 0 && (-xs == val || (xs == -128i8 && val == 127i8)) }
}







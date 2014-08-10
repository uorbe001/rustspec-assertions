# Rustspec-assertions

This is an attempt to port a syntax similar to rspec to rust.

I find the errors rust's built-in assert! gives pretty limited, and I personally like this sort of syntax better so I decided to start this as learning exercise.

## Usage

Add this as a dependency to your `Cargo.toml` and run `cargo build`:

```
[dependencies.rustspec_assertions]
git = "https://github.com/uorbe001/rustspec-assertions.git"
```

Now you should be able to use these assertions in your tests by 'using' them:

```
#[phase(plugin, link)] extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_le, eq, be_lt, be_gt, be_ge};

#[test]
fn be_le_int_test() {
    expect(1i).to(be_le!(2i));
}

#[test]
fn eq_f64_test() {
    expect(1.1f64).to(eq!(1.1f64));
}
```

The crate relies on macros to be able to report better errors, so you'll need to add this to your test.rs, lib.rs or main.rs file:

```
#![feature(phase)]
```

For a complete list of matchers and more examples, please check the [tests](tests/).

## Work in progress

Please be aware this is work in progress and I'm a total rust newbie, so there may be errors and lots of matchers I'd like to see added as I need them.

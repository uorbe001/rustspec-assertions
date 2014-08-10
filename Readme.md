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
extern crate rustspec_assertions;
use self::rustspec_assertions::{expect, be_le, be_less_or_equal_to, eq, equal, be_lt, be_gt, be_ge};

#[test]
fn be_le_int_test() {
    expect(1i).to(be_le(2i));
}

#[test]
fn be_le_f64_test() {
    expect(1.1f64).to(be_le(1.1f64));
}
```

For a complete list of matchers and more examples, please check the [tests](tests/).

## Work in progress

Please be aware this is work in progress and I'm a total rust newbie, so there may be errors and lots of matchers I'd like to see added as I need them.

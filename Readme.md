# Rustspec-assertions ![build status](https://travis-ci.org/uorbe001/rustspec-assertions.svg?branch=master)

This is an attempt to port a syntax similar to rspec or chai to rust.

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
use self::rustspec_assertions::{expect, be_le, eq, be_lt, be_gt, be_ge, contain, be_true, be_false, be_some, be_none};

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

### Matchers

Here are some of the matchers already implemented:

```
expect(2i).to(eq!(2i));
expect(2i).not_to(eq!(3i)); // not_to works with all the matchers
expect(3i).to(be_gt!(2i));
expect(3i).to(be_ge!(3i));
expect(2i).to(be_lt!(3i));
expect(2i).to(be_le!(2i));
expect(vec![1i, 2i]).to(contain!(2i));
expect(true).to(be_true!());
expect(false).to(be_false!());
expect(Some(1i)).to(be_some!());
expect(None::<int>).to(be_none!());
```

For a complete list of matchers and more examples, please check the [tests](tests/).

# Rustspec Note
This is designed to work with [rustspec](https://github.com/uorbe001/rustspec) so I might end up making decisions further down the line to improve error reporting which might not be ideal for other options.

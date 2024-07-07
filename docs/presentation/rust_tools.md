In this section, we will see supported tools by Rust.

## Static code analyzer

### Clippy

It just work like all static code analyzer use in C/C++. 
And this tool also provide hint to correct way of wring rust code.

```bash
warning: this expression creates a reference which is immediately dereferenced by the compiler
  --> src/code_gen.rs:87:80
   |
87 |                 let _evt_func = self._create_state_event_process_function(_st, &_e);
   |                                                                                ^^^ help: change this to: `_e`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_borrow
   = note: `#[warn(clippy::needless_borrow)]` on by default
```

For more about this tool refer its [document here](https://doc.rust-lang.org/clippy/index.html)

### geiger

[cargo-geiger](https://crates.io/crates/cargo-geiger).

This tool provide statistic about unsafe code. This can help to monitor unsafe code addition in project.

### Code Coverage

We will be using [grcov](https://github.com/mozilla/grcov?tab=readme-ov-file#example-how-to-generate-source-based-coverage-for-a-rust-project)

Execute below command

```rust
rustup component add llvm-tools-preview
export RUSTFLAGS="-Cinstrument-coverage"
cargo build
export LLVM_PROFILE_FILE="profile-%p-%m.profraw"
cargo test
```

Use below command to get coverage report.

```rust
grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage/
```

## Security Audit Tool

Rust support audit tool call [cargo-audit](https://crates.io/crates/cargo-audit).
This use public repository on github that track down vulnerability. 
This is highly depend on open source reporting.

## Testing Tools

#### Unit Test

This is as we seen in Introduction presentation.

```rust
    #[test]
    fn test_add_1() {
        let mut out : u32 = 0;
        out = add(2,3);
        assert_eq!(out , 5);
    }
```

#### Mocking Test

[mockall](https://crates.io/crates/mockall)

This utility can be use to perform mocking to test various scenarios.

#### Test Report


#### Benchmark Test

For benchmark, best tool is [Criterion](https://docs.rs/criterion/latest/criterion/). 

> TODO: Need to explore more.

#### Fuzz Test

[cargo-fuzz](https://rust-fuzz.github.io/book/introduction.html)

This plugin use to perform fuzzing base testing.

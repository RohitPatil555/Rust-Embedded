# Rust Tools

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

## Security Audit Tool

Rust support audit tool call [cargo-audit](https://crates.io/crates/cargo-audit).
This use public repository on github that track down vulnerability. 
This is highly depend on open source reporting.

## Testing Tools

#### Enable Sanitization

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

#### Benchmark Test

#### Fuzz Test

#### Test Report

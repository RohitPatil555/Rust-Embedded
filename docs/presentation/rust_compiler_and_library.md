In this section, we will look at compiler and library structure of rust.

## Compiler High level View.

For more detail please refer to [link](https://rustc-dev-guide.rust-lang.org/overview.html)

Below diagram show visualization of compiler pipeline.

![Compiler Pipeline](img/Compiler.png)

## Rust Library View

* Core Library - This contain all feature of rust  programming and basic functionality required for bare metal programming.
* Standard Library - Standard library develop over core library and use system API call for some operation. It also contain additional utility develop for OS. e.g. opening and creating file.
* Thirdparty libraries - They provide higher level of abstraction e.g. UI programming library. They develop over standard library, or some got develop over core library for embedded purpose.

![Library Structure](img/Library.png)


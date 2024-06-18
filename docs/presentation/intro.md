---
theme: default
paginate: true
transition: slide 1.2s
class:
    invert
---

<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->
<style scoped>section{font-size:15px;}</style>

### <!-- fit --> Introduction

Rust Programming Language 

--- 

## Rust History

- 2006 : Lift failure cause Graydon Hoare develop Rust language.
- 2009 : Mozilla sponcer language.
- 2015 : First stable release 
- 2021 : Rust foundation.

---

<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->
<style scoped>section{font-size:15px;}</style>

### <!-- fit --> Why Rust ?

---
<!-- 
_color: white
-->

# C/C++ Issue

Microsoft and Google performed some exercise on C/C++ code and come up with suprising result. 70% Security issues are type of memory safety problem. 

- Microsoft : https://msrc.microsoft.com/blog/2019/07/a-proactive-approach-to-more-secure-code/
- Google: https://www.chromium.org/Home/chromium-security/memory-safety/

---

## CISA Article 

As per below article from CISA.
https://www.cisa.gov/news-events/news/urgent-need-memory-safety-software-products

Ther are two possible solution suggested to mitigate memory safety issue
- Hardware : hardware will track allocation and avoid memory releated solution.
- Software : Memory safe language.

---

### Why Rust ? Ans

Rust make sure memory safety scenario get handle at compile time. 

Hence it can be a software solution as per CISA article.

---

## How Rust Solve this problem ?

Let see some example in C/C++ to understand more about Rust.

---
<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->
<style scoped>section{font-size:15px;}</style>

##### <!--fit--> Example 1

---

### What is issue in below code ?

---

### Rust solution

---

<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->
<style scoped>section{font-size:15px;}</style>

##### <!--fit--> Example 2

---

### What is issue in below code ?

---

### Rust solution

---

## Rust Language Feature

* Memory Safety : Ownership, Borrowing and Lifetime
* Object-oriented Programming : trait concept
* Functional Programming : Closure
* Pattern Matching : if let, match and macros.
* Low-Code / Domain Specific Language : procedure macros.
* Asynchronous Programming.

---
<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->
<style scoped>section{font-size:15px;}</style>

### <!--fit--> Let start with some basic tutorial

---

## Development Environment Setup

- Install Rust : Link 
- Use vscode or vim 

---
<style scoped>section{font-size:20px;}</style>

# Hello World Example

- Execute below command to create project
```bash
cargo new hello
cd hello
```
- Generated source file src/main.rs. 
```rust
fn main() {
    println!("Hello, world!");
}
```
- Build Code
```bash
cargo build
```
- Run Code
```bash
cargo run
```

---
<style scoped>section{font-size:15px;}</style>

# Unit Test environment

- Add new api in src/main.rs
```rust
fn add(x : u32, y: u32) -> u32 {
    let z = x + y;
    z
}
```

- Add test code in src/main.rs
```rust
mod tests {
    use super::*;

    #[test]
    fn test_add_1() {
        let mut out : u32 = 0;
        out = add(2,3);
        assert_eq!(out , 5);
    }
}
```

- Run test
```bash
cargo test

running 1 test
test tests::test_add_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

---

# Exercise

Try to write code to add 3 number along with positive and negative test case.

---
<!-- 
_class: lead 
_backgroundColor: black
_color: blue
-->


# <!--fit--> ThankYou

---
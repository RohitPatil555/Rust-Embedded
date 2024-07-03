---
theme: uncover
paginate: true
transition: slide 1.2s
text-align: left
style: |
  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }
---

### <!-- fit --> Introduction

Rust Programming Language 

--- 

## Rust History

- 2006 : Lift failure cause "Graydon Hoare" develop Rust language.
- 2009 : Mozilla sponsored language.
- 2015 : First stable release 
- 2021 : Rust foundation.

---

# Why Rust ?

---

# C/C++ Issue

Microsoft and Google performed some exercise on C/C++ code and come up with suprising result. 70% Security issues are due to memory safety problem. 

- Microsoft : https://msrc.microsoft.com/blog/2019/07/a-proactive-approach-to-more-secure-code/
- Google: https://www.chromium.org/Home/chromium-security/memory-safety/

---
<!-- text-align: left -->

## CISA Article 

https://www.cisa.gov/news-events/news/urgent-need-memory-safety-software-products

Here they suggested two possible solution to mitigate memory safety issue
- Hardware : hardware to track memory allocation, boundary and free.
- Software : Memory safe language.

---
<style scoped>section{font-size:28px;}</style>

## TIOBE Index

- https://www.tiobe.com/tiobe-index/
- They track languages base on following criteria at high level.
    - Popularity of programming language
    - Number of skilled engineers world-wide
    - Courses and third party vendors
- Source of Information
    - Popular web sites Google, Amazon, Wikipedia, Bing and more than 20 others
    - https://www.tiobe.com/tiobe-index/programminglanguages_definition
- Rust Language
    - Current rank is at 17.
    - Last year same month rank was at 20.

---

## How Rust solve memory related issues ?

Let see some example in C/C++ to understand more about Rust.

---

# Example 1

---

### What is issue in below code ?

```c
void print_x(int *x)
{
    printf("Value : %d \n", *x);
}

int main()
{
    int * x = (int *)malloc(sizeof(int));
    assert(x != NULL);
    *x = 10;
    print_x(x);
    return 0;
}
```
---

### Issue : Memory Leak

```
==27985== LEAK SUMMARY:
==27985==    definitely lost: 4 bytes in 1 blocks
==27985==    indirectly lost: 0 bytes in 0 blocks
==27985==      possibly lost: 0 bytes in 0 blocks
==27985==    still reachable: 0 bytes in 0 blocks
==27985==         suppressed: 0 bytes in 0 blocks
```

---

### Rust solution

```rust
fn test1(k: Box<u32>) {
    println!("value of k {}", k);
}

fn main() {
    let mut m = Box::<u32>::new(0);
    *m = 10;
    test1(m);
}
```

###### No Memory Leak

```
==29399== All heap blocks were freed -- no leaks are possible
```

---

# Example 2

---

### What is issue in below code ?

<div class="columns">
<div>

```c++
class test2 {
    int * x;
public:
    test2() {
        x = new int(5);
    }

    void update_val(int val) {
        *x = val;
    }

    void print_vals(char * text) {
        cout << text << " => " << *x << endl;
    }

    ~test2() {
        delete x;
    }
};
```

</div>
<div>

```c++
void update_test2(test2 t2) {
    t2.update_val(10);
    t2.print_vals("update_test2");
}

int main() {
    test2 t1;
    update_test2(t1);
    t1.update_val(22);
    t1.print_vals("main");
    return 0;
}
```

</div>
</div>

---

### Issue: Memory use after free

Output of code shows value got updated to 22 event x buffer got free in destructor.

```
update_test2 => 10
main => 22
free(): double free detected in app
Aborted (core dumped)
```

---

### Rust solution

<div class="columns">
<div>

```rust
#[derive(Debug)]
struct Foo {
    x : Box<u32>,
}

impl Foo {
    fn new() -> Self {
        Foo {
            x : Box::<u32>::new(10),
        }
    }

    fn update_val(&mut self, val : u32) {
        *self.x = val;
    }
}

fn update_foo(mut x2 :Foo) {
    x2.update_val(10);
    println!(" value : {:?} ", x2);
}

fn main() {
    let mut x1 = Foo::new();
    update_foo(x1);
    x1.update_val(22);
    println!(" value : {:?} ", x1);
}
```

</div>
<div>

### Compiler Error

```
error[E0382]: borrow of moved value: `x1`
  --> src/main.rs:27:5
   |
25 |     let mut x1 = Foo::new();
   |         ------ move occurs because `x1` has type `Foo`, which does not implement the `Copy` trait
26 |     update_foo(x1);
   |                -- value moved here
27 |     x1.update_val(22);
   |     ^^ value borrowed here after move
   |
note: consider changing this parameter type in function `update_foo` to borrow instead if owning the value isn't necessary
  --> src/main.rs:19:23
   |
19 | fn update_foo(mut x2 :Foo) {
   |    ----------         ^^^ this parameter takes ownership of the value
   |    |
   |    in this function

For more information about this error, try `rustc --explain E0382`.
```

</div>
</div>

---

## Rust Language Feature

* Memory Safety : Ownership, Borrowing and Lifetime
* Object-oriented Programming : trait concept
* Functional Programming : Closure
* Pattern Matching : if let, match and macros.
* Low-Code / Domain Specific Language : procedure macros.
* Asynchronous Programming.

---

# Let start with Rust

---

## Setup Development Environment

- [Rust Playground](https://play.rust-lang.org/) : To try out
- Install Rust : [Link](https://www.rust-lang.org/tools/install) 
- Use vscode with [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) or neovim 
- Use rust-gdb to debug code

---
<style scoped>section{font-size:28px;}</style>

## Hello World Example

<div class="columns">
<div>

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
</div>
<div>

- Build Code
```bash
cargo build
```
- Run Code
```bash
cargo run
```
</div>
</div>

---
<style scoped>section{font-size:28px;}</style>

## Unit Test environment

<div class="columns">
<div>

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

</div>
<div>

- Run test
```bash
cargo test

running 1 test
test tests::test_add_1 ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```
</div>
</div>

---

# Exercise

Write code to add 3 number along with positive and negative test cases.

---

# ThankYou

---
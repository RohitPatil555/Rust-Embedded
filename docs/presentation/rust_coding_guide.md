---
theme: uncover
paginate: true
transition: slide 1.2s
text-align: left
class:
    - invert
style: |
  .columns {
    display: grid;
    grid-template-columns: repeat(2, minmax(0, 1fr));
    gap: 1rem;
  }
---

## <!--fit--> Rust Coding Guide

---

## Rules

- Variable definition
- Variable Ownership, Borrowing and Lifetime
- Trait & Generic
- Closure 
- Asynchronous Programming

---

### Variable Definition

- All variable by default define as constant or read-only
- To make them writeable use _mut_ keyword.
```rust
let x : u32 = 2;
x = 4; // Not allowed by compiler

let mut x : u32 = 2;
x = 4; // allowed

let ptr: *const u32 = &x; // pointer to const variable
let ptr: *mut u32 = &mut x; // pointer to mutable variable

```
---

#### Variable Ownership, Borrowing and Lifetime

- In below statement, we will see how x variable ownership of u32 get transfer.
```rust
let x : u32 = 10;
let y : u32;

y = x; // ownership transfer to y. Now x is invalid.

func1(y); // ownership transfer to y. Now y is invalid.  
```

---

#### Variable Ownership, Borrowing and Lifetime

- lifetime definition
```rust
fn main() {
    let z : u32 = 0;
    let x : u32 = 10;-------------------------+-
    {                     --+                 x lifetime
                            y lifetime        |
        let y : u32 = 20; --+-----------------+-
    }
}
```

---

### Trait

- This act like abstract class as in c++ but not exact same. 
    - It not have _vpointer_ concept in Rust.
    - But it can be derived from base trait.
- trait can be seen as below:
```rust
trait Name {
    fn func1(&self) -> String;
    fn func2(&self) -> String {
        String::from("Hello Func2")
    }
}
```

---

### Trait application

- Trait are like functional abstraction and they applied to custom data structure as shown below.

```rust
struct Foo {};

impl Name for Foo {
    fn func1(&self) -> String {
        String::from("Foo ")
    }
}
```

---

### Trait from standard library

- Core library use to control object behavior (e.g. copy constructor, destructor and debug)

```rust
#![derive(Debug,Copy)] // here Debug Copy macro expand same like drop shown below.
struct Cat {};

impl Drop for Cat {
    fn drop(&mut self) {
        // cleanup logic
    }
}
```
---

### Trait Generic

-  Similar to C++ template, but only applied on trait.

<div class="columns">
<div>

```rust
pub trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}
```
</div>
<div>

```rust
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
```
</div>
</div>

---

### Closure

- Similar to C++ lambda expression, can be expanded as inline function.
- And also can work like functional programming, allocated on heap memory (Rust take care free part)
- Three type of closure
    - FnOnce, FnMut, Fn 
- Example
```rust
let square = |x| x * x;
assert_eq!(square(5), 25);
```

---

### Function Pointer

- Different from closure but can be use like Fn closure.
- function pointers can vary based on what ABI they use (e.g. _extern "C" fn(u32) -> u32_ )

```rust
    fn add_one(x: usize) -> usize {
        x + 1
    }

    let ptr: fn(usize) -> usize = add_one;
    assert_eq!(ptr(5), 6);
```
---

### Asynchronous Programming

- Below code show how asynchronous program look like and work.

```rust
async fn fun1() {
    ...
    fun2().await; // this block fun1 and start execute fun2. 
    ... // fun1 got resumed.
}

async fn fun2() {
    ...
} // this will unblock fun1.

```
- It look similar to cooperative scheduler.

---

# Thankyou

---

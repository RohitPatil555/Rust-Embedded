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

## <!--fit--> Rust Features

###### at code level

---

## Contain

- Variable definition
- Variable Ownership, Borrowing and Lifetime
- Trait & Generic
- Closure 
- Asynchronous Programming

---

### Variable Definition

```rust
let x : u32; // declaration of variable x. 
x = 4; // allowed
x = 5; // Not-allowed

let x : u32 = 2; // all variable by default immutable type.
x = 4; // Not allowed by compiler

let mut x : u32 = 2;
x = 4; // allowed

let ptr: *const u32 = &x; // pointer to const variable
let ptr: *mut u32 = &mut x; // pointer to mutable variable

```
---

#### Variable - Ownership

```rust
let x : Box::<u32>::new(10);
let z : Box::<u32>::new(20);

let y = x; // ownership transfer to y. Now x is invalid.

// x is not available.

func1(y); // ownership transfer to func1. Now y is invalid.  

// y is not available.

```

---

#### Variable - Borrowing

```rust
fn func2(k : &Box<u32>) {
    println!("Borrowed z {}", k);    
}

fn main() {
    let z = Box::<u32>::new(10);
    func2(&z);
    println!("owner z {}", z);
}
```

---

#### Variable - Lifetime

```rust
fn get_min<'a,'b>(x : &'a Box<u32>, y: &'b Box<u32>) -> &'a Box<u32> {
    if x > y {
        y
    } else {
        x
    }
}

fn main() {
    let x = Box::<u32>::new(10); // lifetime of x start
    let z : &Box<u32>; // lifetime of z start
    {                            
        let y = Box::<u32>::new(20); // lifetime of y start
        z = get_min(&x, &y); // return y - cause lifetime compilation error.
    } // y lifetime End        
    
    println!("Value of x : {}", *z); 
} // x and z lifetime End
```

---

### Trait

- This act like abstract class as in c++ but not same. 
    - no _vpointer_ concept in Rust.
    - It can be derived from base trait.
    - trait not take memory space.
    - For dynamic dispatch, it need allocation of object/instance.

```rust
trait Name {
    fn func1(&self) -> String;
}
```

---

### Trait : Dynamic Dispatch

<div class=columns>
<div>

```rust
trait Sound {
    fn make_sound(&self) -> String;
}

struct Cat;
struct Dog;

impl Sound for Cat {
    fn make_sound(&self) -> String {
        format!("meow meow ..")
    }
}

impl Sound for Dog {
    fn make_sound(&self) -> String {
        format!("woof woof ..")
    }
}

fn get_animal(x: u32) -> Box<dyn Sound> {
    if x == 1 {
        Box::new(Cat{})
    } else {
        Box::new(Dog{})
    }
}

fn main() {
    let mut animal = get_animal(1);
    println!("1 -> {}", animal.make_sound());
    animal = get_animal(2);
    println!("2 -> {}", animal.make_sound());
}
```

</div>
<div>

###### output

```bash
1 -> meow meow ..
2 -> woof woof ..
```

</div>
</div>


---

### Trait from standard library

- Rust library use to control object behavior (e.g. copy constructor, destructor and debug)

```rust
#![derive(Debug,Copy)] // here Debug Copy macro expand same like drop shown below.
struct Test {};

impl Drop for Test {
    fn drop(&mut self) {
        // cleanup logic
    }
}
```
---

### Trait Generic

<div class="columns">
<div>

```rust
trait Flame {
    fn get_color(&self) -> String;
}

// Generic function.
fn flame_color<T> (animal : T) 
    where T: Flame
{
    println!("Color of Flame : {}", animal.get_color());
}

struct GasolineCar{}
impl Flame for GasolineCar {
     fn get_color(&self) -> String {
        format!("Blue")
     }
}
struct DieselCar{}
impl Flame for DieselCar {
     fn get_color(&self) -> String {
        format!("Yellow")
     }
}
fn main() {
    let a = GasolineCar{};
    let d = DieselCar{};
    
    flame_color::<GasolineCar>(a);
    flame_color::<DieselCar>(d);
}
```

</div>
<div>

-  Similar to C++ template, but with trait.

###### output

```bash
Color of Flame : Blue
Color of Flame : Yellow
```

</div>
</div>

---

### Closure

- Similar to C++ lambda expression, can be expanded as inline function.
- And also can work like functional programming, allocate on heap memory (Rust take care free part)
- Three type of closure
    - FnOnce, FnMut, Fn 
- Example
```rust
let square = |x| x * x;
assert_eq!(square(5), 25);
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

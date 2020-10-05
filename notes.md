#### cargo 
- `cargo doc --open` provides documentation for all crates

## common programming concepts 

#### variables and mutability
- by default, all variables are immutable
    - `let x = 5; . . . x = 6;` is impossible
    - `let mut x = 5; ... x = 6;`
- constants
    - you aren't allowed to use `mut` on `const`s, they're always immutable
    - `const MAX_POINTS: u32 = 100_000`
- shadowing
    ```rust
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    ```
    - this is different from marking a variable as `mut`
    - after these transformations, `x` will still be immutable
    - via shadowing, we can also change the type of the variable
    ```rust
    // allowed
    let spaces = "   ";
    let spaces = spaces.len();

    // not allowed
    let mut spaces = "   ";
    spaces = spaces.len();
    ```
        - you cannot change a variables type

#### scalar types
    - represents a single value
    - 4 primary types: `integers`, `floats`, `booleans`, `chars`
- integer types
    - `u32`
        - `u` for unsigned
        - 32 for bit space
    - visual seperator
        - `98_222` to represent 98,222
- floating point types
    - `f32`: single point precision
    - `f64`: double precision
- boolean type 
    - `let f: bool = false;`
- character type
    - specifically denoted with single quotes
    - `let z = 'ℤ';`
    - 4 bytes, represents utf-8

#### compound types
- grouping two variables into one (tuples, arrays, etc.)
- tuple
    - fixed length
    - `let tup: (i32, f64, u8) = (500, 6.4, 1);`
    - access via a period
    - `tup.0` -> 500
- array
    - all same type
    - >useful when you want your data allocated on the stack rather than the heap
    - `let a: [i32; 5] = [1, 2, 3, 4, 5];`
        - 5 elements of type `i32`
    - out of bounds exceptions are not caught at compile time
        -  does *not* allow you to access memory outside of the array

#### functions
- snake case
- location of the functions don't matter (unlike c)
- must declare the type of each parameter
- function definitions are also statements
```rust
let x = 5;

let y = {
    let x = 3;
    x + 1
};
```
    - `x+1` does not have a semicolon after it; it is an expression not a statement
        - expressions return, statements do not

- return values
    - `fn five() -> i32 { 5 }`
    - if `5` had a semicolon after it, this wouldn't return

## if expressions
- `if expression { } else { }`
- condition _must_ bee a bool (unlike JS)
- if let
    - `let number = if condition { 5 } else { 6 };`
        -  these types cannot be mismatched

## loops
```rust
loop {
    . . .
    break;
}
```
```rust
let mut counter = 0;

let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;
    }
};

println!("The result is {}", result);
```
- conditional loops
    - `while expression { }`
- for loop
```rust
let a = [10, 20, 30, 40, 50];

for element in a.iter() {
    println!("the value is: {}", element);
}

for number in (1..4).rev() {
    println!("{}!", number);
}
```

# understanding ownership
- rust does not have a garbage collector

## what is ownership?

### the stack and the heap
- the stack stores values as LIFO
- *known & fixed size* -> stack
- *unknown/variable size* -> heap
- the heap is unorganized
    - you allocate onto the heap and are returned a pointer
        - this pointer can be stored on the stack
- pushing on the stack is faster than allocating on the heap
    - to allocate on the heap, the allocate has to search for free space
- accessing data from the heap is slower because you have to follow a pointer
- when you code calls a function, the values passed and the local variables get put on the stack. when the function is over, they're popped off

### ownership rules
- each value has a variable called it's *owner*
- there can only be 1 owner at a time
- when the owner goes out of scope, the value will be dropped

### variable scope

```rust
{
    let s = "hello"; // s is valid
    // do stuff with s
}
// scope is over; s is no longer valid
```

### The `string` type
- `String` is a complex data type
    - this is different from a string literal
- `let s = String::from("hello");`
- `String` can be mutated but literals cannot

### Memory and allocation
- for a string literal, we know the content and size at compile time
    -  it can be stored on the stack
- `String` is a mutable, growable, piece of text
    - it must be stored on the heap
- instead of using `free` or `allocate`, memory is automatically returned once the variable goes out of scope
    - internally, rust calls `drop`

### variables and data: move

```rust
let x = 5;
let y = x;
```
1. bind the value `5` to `x`
2. make a copy of the value in `x` and bind it to `y`

```rust
let s1 = String::from("hello");
let s2 = s1;
```

s1 contains: `{ ptr, len, capacity }`
    - `ptr` points to the String in the heap
    - `len` is the number of bytes String is currently using
        - this comes from the allocator

s2 contains: `{ ptr, len, capacity }`
    - `ptr` address is copied from s1 (stack)
    - `len` and `capcity`'s values are copied from s1 (stack)
    - the data on the heap is NOT copied

- when `s1` and `s2` go out of scope, they will *both* try to free the same memory
    - this is called a double free error
- to prevent a double free error, rust considers `s1` invalid as soon as `s2` is declared

```rust
let s1 = String::from("hello");
let s2 = s1;

println!("{}, world!", s1);
```

> this will not compile: move occurs because `s1` has type `std::string::String`, which does not implement the `Copy` trait

instead of calling this a "shallow copy", rust calls this a `move`

> s1 was moved into s2

### variables and data: clone

this is a "deep copy" of data

```rust
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {}, s2 = {}", s1, s2);
```

>here, the heap data _does_ get copied into a new location for s2

#### stack-only data: copy

```rust
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```

because integers have a known size at compile time, copies of the value are quickly made

the `Copy` trait can be placed on data types stored on the stack
    -  `Copy` cannot be applied if the type has implimented the `Drop` trait

what types are `Copy`?
- all integer types
- all boolean types
- all chars
- tuples iff they only contain types that are also copy
    - e.g. `(i32, i32)` is `Copy` but `(i32, String)` is not

### ownership and functions

```rust
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
    // but i32 is Copy, so it’s okay to still
    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
// special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```
using `s` after the call to `takes_ownership` will throw a compile-time error

### return values and scope

returning values can also transfer ownership

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
    // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
    // takes_and_gives_back, which also
    // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
// moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
    // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

- returning a `String` gives the ownership to the caller
- taking `String` and returning `String` will take and return the param
- When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless the data has been moved to be owned by another variable.

## References and Borrowing
taking in a reference to an object:

```rust
fn main() {
    let s1 = String::from("hello!");
    let len = calcuate_length(&s1);
    println!("the length of '{}' is {}.'", s1, len);
}

fn calcuate_length(s: &String) -> usize {
    s.len()
}
```

- `s` refers to the value of `s1` but does not own it
    - because it does not own it, the value wont be dropped but the ref will
- this is called "borrowing"
- immutable by default

### mutable references

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

- *note* you can only have 1 mutable reference to a piece of data at a time

```rust
let r1 = &mut s;
let r2 = &mut s;
```

> this is not allowed

this check can prevent a data race at compile time

issues such as:
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There’s no mechanism being used to synchronize access to the data.

you *can* have more than one immutable reference at a time if no mutable references are present

```rust
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // r1 goes out of scope here, so we can make a new reference with no problems.

let r2 = &mut s;
```
> this is okay and works because of the scopes

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM

println!("{}, {}, and {}", r1, r2, r3);
```
> does not work because you can't have both immutable and mutable refs at the same time

```rust
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// r1 and r2 are no longer used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

### Dangling references
- can be prevented with a compile-time error

```rust
fn dangle() -> &String { // dangle returns a reference to a String
    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!
```
> when the code of dangle is finished, s will be deallocated

to solve this, we should return `s` in full to transfer ownership

## The slice type

a slice lets you reference a contiguous sequence of elements with no ownership

```rust
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() { // enumerating
        if item == b' ' { // b' ' represents a byte string
            return i;
        }
    }

    s.len()
}
```
we could do this, but it's not great...

if any of the following happens, we're screwed:
- the callee mutates the string
- the string falls out of scope
- the string is cleared
- etc.

###String slice

a reference to a part of a string

```rust
let s = String::from("hello world");

let hello = &s[0..5];
let world = &s[6..11];
```

with the `..` range syntax:
- `0..2` is equivalent to `..2`
- `3..len` is equivalent to `3..`
- `0..len` is equivalent to `..`

string slices *must occur at valid utf-8 character boundaries*

### getting the first word with string slices

```rust
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

with slices, the following now breaks

```rust
fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
```
> word derrives from an immutable borrow, it is then used when printing 
> because of this, s.clear() (a mutable borrow) cannot occur

- string literals are slices
    - they're slices pointing to immutable memory  


# Using structs to structure related data

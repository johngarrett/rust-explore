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

## defining and instantiating structs

```rust
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
```
to use a struct, we have to create an instance

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

user1.email = String::from("another@a.com");
```

we can change this _because_ the instance is mutable
    - note: the entire instance must be mutable, rust doesn't allow us to mark certain fields as mutable

### field init shorthand

```rust
let t = User {
    email,
    username,
    active: true,
    sign_in_count: 1,
}
```

> because the parameter names are the same as the argument names, you don't have to repeat them

### creating instances from other instances

this:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    active: user1.active,
    sign_in_count: user1.sign_in_count,
};
```

is the same as:

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername567"),
    ..user1
};
```

### Tuple Structs

structs that look similar to tuples but have the benefit of naming

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);
```

### unit-like structs

these are structs without any fields.

they behave similarly to `()`.

these can be useful if you want to impliment a trait on some type

### ownership of struct data

instances of a struct should own all it's data; the data should be valid as long as the struct is valid

> this is the difference between using `&str` and `String`

to store refs in structs, you need to make use of *lifetimes*

## Example using structs

```rust
fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "The area of a rectangle is {} square pixels.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

it would be more readable to group the width and height together. The parameters are related but that's not expressed anywhere in the program

```rust
fn main() {
    let rect = (30, 50);
    println!(
        "The area of a rectangle is {} square pixels.",
        area(rect)
    );
}

fn area(dimensions: (u32, u32) -> u32 {
    dimensions.0 * dimensions.1
}
```

this adds better structure but is actually less readable. we don't know what is at .0 and what is at .1

```rust
struct Rectangle {
    width: u32,
    height: u32,
}
fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
```

this is more readable and provides clarity to the programmer

we can go a step further though...

if `Rectangle` is printed, it should display its area

> the best way to do so is by implementing `std::fmt::Display`
> this is out of scope right now

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rect1 is {:?}", rect1);
}
```

this will print out: `rect1 is Rectangle { width: 30, height: 50 }`

## Method Syntax

methods are similar to functions but they're defined within the context of a struct (or enum or trait...)

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

`impl` is called an implementation block. (i think it's similar to extensions in swift)

methods can take ownership of self or borrow self mutably and immutably

`impl` is a good place to put all the tings we can do with an  instance of a type

### the `->` operator

in c/c++, `.` is used when directly calling a method and `->` is used when calling a method on a pointer.

rust has *automatic referencing and dereferencing*, calling a method is a place where rust has this behavior

`object.something()`
rust will automatically add in `&`, `&mut`, or `*` so `object` matches the signature of the method

```rust 
p1.distance(&p2);
(&p1).distance(&p2);
```
> these are the same

### methods with more parameters

```rust
impl Rectangle {
    fn can_hold(&self, other_rect: &Rectangle) -> bool {
       self.width >= other_rect.width 
            && self.height >= other_rect.height 
    }
}
```

### Associated Functions

associated functions _don't_ take self in as a parameter

these are often used for constructors (think php)

```rust
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

to call this, we would do the following:
`let sq = Rectangle::square(3);`


### multiple `impl` blocks

```rust
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
```

there is no real raeson to seperate these methods but the syntax is valid

# Enums and Pattern matching

## defining an enum

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

the variants of an enum are namespaced under its identifier

`fn route(ip_kind: IpAddrKind) {}`
> enums are arguments

```rust
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
};

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
};
```

the code above can be simplified into a single enum

```rust
enum IpAddr {
    V4(String),
    V6(String),
}

let home = IpAddr::V4(String::from("127.0.0.1"));
let loopback = IpAddr::V6(String::from("::1"));
```

each variant can have a different type and amount of data

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);

let loopback = IpAddr::V6(String::from("::1"));
```

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

- `quit` has no data associated with it
- `movie` includes a struct
- `write`includes a single string

we can also define methods onto a struct

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## the option enum and its advantages over null values

optionality in rust is implimented as an enum

```rust
enum Option<T> {
    Some(T),
    None,
}
```

the option enum is automatically brought in with the prelude; no need to include it

```rust
let some_number = Some(5);
let some_string = Some("a string");

let absent_number: Option<i32> = None;
```

> when using None as a value, you have to tell the compiler what T is

```rust
let x: i8 = 5;
let y: Option<i8> = Some(5);

let sum = x + y;
```

> this does not work; rust doesn't automatically unwrap y

## the match control flow operator

similar to a traditional `switch` statement 

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

in an `if` statement, the condition must be boolean. with a `match`, it can be any type

### Patterns that bind to values

```rust
#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

. . .
value_in_cents(Coin::Quarter(UsState::Alaska))
```

### matching with Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

- `i` binds to the value contained in `Some`
- *`matches` must be exhaustive*

### the `_` Placeholder

```rust
let some_u8_value = 0u8;
match some_u8_value {
    1 => println!("one"),
    3 => println!("three"),
    5 => println!("five"),
    7 => println!("seven"),
    _ => (),
}
```

the `_` will match any value. the `()` is just a unit value, basically saying it does nothing

## control flow with `if let`

`if let` takes a pattern and expression seperatated by an equal sign

```rust
let mut count = 0;
match coin {
    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    _ => count += 1,
}

// versus

let mut count = 0;
if let Coin::Quarter(state) = coin {
    println!("State quarter from {:?}!", state);
} else {
    count += 1;
}
```

# Managing growing projects with packages, crates and modules

## Packages and Crates

- A `crate` is a binary library
- A `package` is one or more crates that provide a set of functionality

## Defining Modules to Control Scope and Privacy

- The `use` keyword brings a path into scope
- the `pub` keyword makes them public

`cargo new --lib`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
```
> src/lib.rs

- a module is defined by starting with the `mod` keyword then specifing the name of the module

- `hosting` and `serving` are both children to `front_of_house`

## paths

two forms:
1. an `absolute` path starts from a crate root
    - uses a crate name or literal crate
2. a `relative` path starting from the current module
    - uses `self` and `super`

```rust
mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}
```
> this doesn't work, `hosting` MUST be declared as public

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

> both the `mod` and all its `fn`s need to be declared as public

### Starting relative paths with super

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}
```
> the `super` keyword allows us to go back to the parent module of `back_of_house`

### Making structs and enums public

- if you mark a struct as `pub`, all of it's fields will still be private
- if we mark an enum as `pub`, all of it's variants are public

```rust
mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
}
```

```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## bringing paths in with `use`

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
> we only have to call `hosting::` and not `crate::from_of_house::hosting`

`use` can be used to bring in items via a relative or absolute path

### creating idiomatic use paths

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting::add_to_waitlist;

pub fn eat_at_restaurant() {
    add_to_waitlist();
    add_to_waitlist();
    add_to_waitlist();
}
```

standard library

```rust
use std::collections::HashMap;

fn main() {
    let mut map = HashMap::new();
    map.insert(1, 2);
}
```

### using the `as` keyword

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
```

### re-exporting names with `pub` and `use`

by using `use`, we bring a name into scope for us

if you use `pub use`, all code that calls our code can refer to the same

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

### using external packages

`std` is a crate external to our package

```
[dependencies]
rand = "0.5.5"
```

> Cargo.toml

`use rand::Rng;`

> main.rs

### using nested paths to cleanup `use`

```rust
use std::cmp::Ordering;
use std::io;

// vs
use std::{cmp::Ordering, io};
```

### the glob operator

brings in _all_ public items defined in a path

`use std::collections::*;`

## Separating modules into different files

`pub mod hosting;`

> `src/front_of_house.rs`

using a semi colon after `mod ...` tells rust to load the contents of the module from another file with the same name

```rust
pub fn add_to_waitlist() {}
```

> `src/front_of_house/hosting.rs`


# Common Collections

rust's standard library includes useful data structures called _collections_

three to discuss:
- vector
- string
- hash map

## Storing lists of values with vectors

`let v: Vec<i32> = Vec::new();`

> bc we aren't initalizing with any values, we must specify the type

`let v = vec![1, 2, 3];`

> rust can infer the type

### updating a vector

- must be `mut` able

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

a vector is freed when it goes out of scope, like any other struct

### Reading 

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```

> you can either index or use `.get`

`get` will return an `Option<&T>`

accessing an invalid location with `[]` will cause the program to panic

accessing an invalid location with `.get()` will simply return `None`

```rust
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```

> COMPILER ERROR

the 3rd line is trying to update `v` but it is immutably borrowed by `first`

- if adding a new element caused the vector to be reallocated elsewhere, `first` would point to invalid memory

### Iterating over values

```rust
let v = vec![100, 32, 57];

for i in &v {
    println!("{}", i);
}
```

> iterating with immutable references 

```rust
let v = vec![100, 32, 57];

for i in &mut v {
    *i += 50;
}
```

> iterating with mutable references

### using an enum to store multiple types

```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```

`push`, `pop`, etc. are also valid functions of a vector

## Storing UTF-8 with Strings

### What is a string?

rust only has one string type in the core language, `str`

the `String` type, provided by the standard library, is:
- growable
- mutable
- owned
- UTF-8 complient

other strings also exist, such as `OsString, OsStr, CString, CStr`


### creating a new string

`let mut s = String::new();`

```rust
let data = "initial contents";

let s = data.to_string();

// the method also works on a literal directly:
let s = "initial contents".to_string();
```

> using `to_string` to create a `String` from a string literal

`let s = String::from("initial contents");`

### Updating a String

a `String` can grow in size and it's contents can change

```rust
let mut s = String::from("foo");
s.push_str("bar");
```

> appending a string slice to foo

`push_str` takes ownership of any string passed to it

`push` takes a single character and adds it to the string

### Concatenation with `+` or `format!`

```rust
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
```

> `s1` is no longer valid as it is now owned by `s3`

you can only add a `&str` to a `String`

in this example, rust `coerces` the `&String` of `s2` into a `&str`

```rust
let s1 = String::from("tic");
let s2 = String::from("tac");
let s3 = String::from("toe");

let s = format!("{}-{}-{}", s1, s2, s3);
```

> `format!` works the same was as `prinln!` but returns a `String`

### indexing into Strings

you *cannot* use `[]` to index a String

internally, `String` is a wraper over a `Vec<u8>`

`let hello = String::from("Hola");`

> this has a length of 4, each letter is 1 byte unicode encoded

`let hello = String::from("Здравствуйте");`

> this is 24 bytes, *not 12*, unicode scalars are used here

the length mismatches are why `[]` indexing does not work


with a string, indexing operations cannot be garunteed to be `O(1)`

### Slicing Strings

you _can_ use `[]` with a range to create a slice

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

> `s` will be a `&str` that contains the first 4 bytes of the string

### iterating

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

> prints each character onto a new line

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

> prints the 18 bytes that make up the `String`


## Hash Maps

`HashMap<K, V>` stores a mapping of keys of type `K` to values of type `V`

### creating a new hash map

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

> we need to first `use` the `HashMap` from the collections

- hashmaps store their data on the heap
- hashmaps are homogenous

```rust
use std::collections::HashMap;

let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let mut scores: HashMap<_, _> =
    teams.into_iter().zip(initial_scores.into_iter()).collect();
```

`HashMap<_, _>` is needed here because `collect` can conform to many different datatypes. using `_`'s, rust is told to infer the type 

### HashMaps and Ownership

types that implement the `Copy` trait, like `i32`, the values are copied into the hash map

for owned values, like `String`, the values will be moved and the hash map will be the owner


```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```

### Accessing values

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```

> `score` has the value that's associated with the Blue team

`.get(_)` returns an `Option<&V>`

you can iterate as such:

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```

### Updating a Hash Map

each key can only have one value associated with it at a time

thus, if we insert a value into a hashmap where a value already exists, the incoming value will overwrite the pre-existing one

```rust
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

> this code will print `{"Blue": 25}`; `10` is gone

#### only insert if the key has no value

```rust
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

> we want to check if the key associated with `yellow` has a value. if it doesn't, we insert 50

> output: {"Yellow": 50, "Blue": 10}

#### updating based on the old value

```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0); // occurances ?? 0
    *count += 1;
}

println!("{:?}", map);
```

> prints: {"world": 2, "hello": 1, "wonderful": 1}

### Hashing functions

by default, `HashMap` used a "cryptographically strong" hashing function

you _can_ switch the hasher by building a hasher that impliments the `BuildHasher` trait


# Error Handling

Most of the time, rust requires you to acknowledge the possibility of an error and take an action at compile time

two types: `recoverable` and `unrecoverable`

recoverable errors have a type `Result<T, E>`
unrecoverable errors call the macro `panic!`

## unrecoverable errors with `panic!`

by default, when a panic occurs, rust will clean up the stack and abort

```
[profile.release]
panic = 'abort'
```

> this would cause the program to abort outright without clearing the stack -- the OS must do so

### using a `panic!` backtrace

```rust
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

> will output: thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'

setting `RUST_BACKTRACE=` to anthing besides zero will give us a backtrace

`RUST_BACKTRACE=1 cargo run`

## Recoverable errors with `Result`

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```
> the File::open method returns a result type

`Result::` is automatically brought into scope by the prelude

### Matching on different errors

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error)
            }
        },
    };
}
```

`io::Error` has a method `kind` that returns an `io::ErrorKind` value

in the code above, we match against the inital result type. If an error occured and it was because the file was `NotFound`, create a new file. After creating a new file, match on its result and continue

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```

> this does the same as the block above, slightly more readable

### `unwrap` and `expect`

this allows us to interact with result types without the verbosity of `match`

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```

> this is similar to `unwrap()` but you are able to provide a good error message

### Propagating Errors

returning the error to the calling code so that it can decide what to do

```rust

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

> The code that calls this will have to handle getting either an `Ok` value or an `Err`

#### the `?` shortcut for propagating errors

```rust
use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

> based off the code block above

the `?` after a `Result` is defined to work very similarly to our previous `match` expression. If the value is `Ok`, the value inside `Ok` will be returned. If the value is an `Err`, the `Err` will be returned

*however*, the `?` operator requires all errors to be of the same type. As long as each error caught implements the `from` function, the `?` will take care of this conversion automatically.

```rust

use std::fs;
use std::io;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

> even shorter way to read it


### the `?` operator + `Result`

`?` can be used on any function that returns `Result`, `Option`, or any type of `std::ops::Try`

the `main` function is special and there are restrictions on what its return type can be

```rust
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```


## To panic! or not to panic!

when code `panics` there is no way to recover. when you chose a `Result` value, you give the callee options to recover

### cases where you have more information than the compiler

it would be appropriate to call `unwrap` when you have some other logic that ensures `Result` will have an `Ok` value -- this isn't something the compiler would pick up on.

`let home: IpAddr = "127.0.0.1".parse().unwrap();`

> parsing a hardcoded string literal, we know `unwrap` will always succeed

### Guidelines for error handeling

panicking is okay when:
- the bad state is not something that's expected to happen
- your code after this point needs to rely on not being in the bad state
- there's not a good way to encode this information in the types you use

however, when failure is expected, a `Result` is more appropriate

## Custom types for validation

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}
```

Now, `Guess` will validate and ensure the value is between `1` and `100` on every initalization

# Generic Types, Traits, and Lifetimes

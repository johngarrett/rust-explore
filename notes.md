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



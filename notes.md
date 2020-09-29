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


## guessing game 

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

#### data types



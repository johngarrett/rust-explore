fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
    println!("The value of 5 is: {}", five());
}

fn five() -> u32 {
    5
}

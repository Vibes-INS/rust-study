fn main() {
    println!("Hello, world!");

    let y = another_function(100_00);
    println!("The value of y is: {}", y);
}

fn another_function(x: i32) -> i32 {
    let y = {
        let x = x + 3;
        x + 1
    };
    println!("another_function {}", y);
    y
}
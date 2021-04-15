fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = 6;
    println!("The value of x is {}", x);

    let spaces = "     ";
    let spaces = spaces.len();

    println!("spaces: {}", spaces);

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let remainder = 43 % 5;

    println!("sum: {}", sum);
    println!("difference: {}", difference);
    println!("product: {}", product);
    println!("quotient: {}", quotient);
    println!("remainder: {}", remainder);

    println!("0.1 + 0.2 = {}", 0.1 + 0.2);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of z is: {}", z);

    let arr = [1, 2, 3, 4, 5];
    let index = 4;
    let element = arr[index];
    println!("The value of element is: {}", element);
}

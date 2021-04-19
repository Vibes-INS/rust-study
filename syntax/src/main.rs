fn main() {
    println!("Hello, world!");
    let number_list = [ 1, 2, 3, 4, 5 ];
    let x = largest(&number_list);
    println!("{}", x);
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> T {
    let mut max = list[0];
    for &item in list {
        if item > max {
            max = item
        }
    }
    max
}
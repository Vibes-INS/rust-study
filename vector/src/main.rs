#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    // part 1
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    for item in &v {
        println!("item: {}", item);
    }

    // part 2
    let v2 = vec![1, 2, 3];
    for item in &v2 {
        println!("item: {}", item);
    }

    // part 3
    let v3 = vec![1, 2, 3, 4, 5, 6];
    let third: &i32 = &v3[2];
    println!("The third element is {}", third);
    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // part 4
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    for item in &row {
        println!("item: {:?}", item);
    }
}
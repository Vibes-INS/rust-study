fn main() {
    part1();
    convert_fahrenheit_to_centigrade(20, 40);
    println!("fibonacci: {}", fibonacci(10));
}

fn part1() {
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if number == 3 {
        println!("condition was true");
    }

    if number != 2 {
        println!("condition was true");
    }

    let number = if number == 3 {
        5
    } else {
        6
    };

    println!("number is {}", number);

    let mut counter = 0;
    let loop_result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The counter loop result is {}", loop_result);

    let mut counter2 = 10;
    while counter2 != 0 {
        counter2 -= 1
    }

    println!("The counter2 is {}", counter2);

    // for
    {
        let arr = [1, 2, 3, 4, 5];
        let mut index = 0;
        let len = arr.len();
        while index < len {
            println!("the  arr[{}] value is: {}", index, arr[index]);
            index += 1;
        }
    }

    let arr = [10, 20, 30, 40, 50];
    for el in arr.iter() {
        println!("the el value is: {}", el);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!");
}

fn convert_fahrenheit_to_centigrade(range_start: i32, range_end: i32) {
    println!("| -------------- | --------------\t|");
    println!("| Fahrenheit     | Centigrade    \t|");
    println!("| -------------- | --------------\t|");
    // base is fahrenheit
    for f in range_start..range_end {
        let c = (f - 32) * 5 / 9;
        println!("| Fahrenheit: {} | Centigrade: {}\t|", f, c);
    }
}

fn fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }

    let mut p = 0;
    let mut q = 0;
    let mut r = 1;

    for _i in 0..n + 1 {
        p = q;
        q = r;
        r = p + q;
    }

    r
}
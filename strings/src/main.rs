fn main() {
    part1();
    part2();
    part3();
}

fn part1() {
    let mut s = String::new();
    s.push_str("create_string");
    println!("{}", s);

    let mut s2 = "initial contents".to_string();
    s2.push_str(": test");
    println!("{}", s2);
}

fn part2() {
    {
        let mut s = String::from("foo");
        s.push_str("bar");
        println!("{}", s);
    }
    {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s1 is {}", s1);
        println!("s2 is {}", s2);
    }
    {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = String::from("") + &s1 + &s2;
        println!("s3 is {}", s3);
    }
}

fn part3() {
    let hello = "Hello World";
    let slice = &hello[0..3];
    println!("slice is {}", slice);
}
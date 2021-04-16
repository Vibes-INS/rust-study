fn main() {    
    let mut s1 = String::from("Hello");
    change(&mut s1);
    let len = calculate_length(&s1);

    println!("The s1 len is: {}", len);

    let s2 = no_dangle();
    let s2_len = calculate_length(&s2);
    println!("The s2 len is: {}", s2_len);
}

fn calculate_length(str: &String) -> usize {
    str.len()
}

fn change(str: &mut String) {
    str.push_str(", World");
}

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
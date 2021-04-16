fn main() {
    let str = String::from("Hello, world!");
    let fitst_word_index = get_fitst_word_index(&str);
    println!("fitst_word_index: {}", fitst_word_index);

    let str2 = String::from("Test Tester String");
    let word = get_first_word(&str2);
    println!("word: {}", word);

    let arr =[1,2,3,4];
    let arr_slice = &arr[1..3];
    for el in arr_slice.iter() {
        println!("el: {}", el);
    }
}

fn get_fitst_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn get_first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
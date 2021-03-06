use std::sync::Mutex;
use std::thread;

fn main() {
    part1();
    part2();
}

fn part1() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6
    }

    println!("m = {:?}", m);
}

fn part2() {
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
use std::{thread, time};

fn sleep_print(name: &str) {
    for i in 1..=3 {
        println!("{}: i={}", name, i);
        thread::sleep(time::Duration::from_millis(1000));
    }
}

fn main() {
    println!("No Threads");
    sleep_print("No Threads");

    println!("Use Threads");
    thread::spawn(|| sleep_print("二郎"));
    thread::spawn(|| sleep_print("三郎"));
    sleep_print("太郎");
}

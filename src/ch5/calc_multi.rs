use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let request_nums = [43, 42, 20, 39, 37, 35, 30];
    let start_time = Instant::now();
    let (tx, rx) = mpsc::channel::<(i64, i64)>();
    for num in request_nums {
        let sender = tx.clone();
        thread::spawn(move || {
            let answer = fib(num);
            sender.send((num, answer)).unwrap();
        });
    }
    let mut job = request_nums.len();

    loop {
        if let Ok((arg, answer)) = rx.recv() {
            job -= 1;
            println!("[結果] fib ({})={} (残り={})", arg, answer, job);
            if job <= 0 {
                show_time(start_time);
                break;
            }
        }
        thread::sleep(Duration::from_millis(300));
    }
}

fn fib(n: i64) -> i64 {
    if n == 1 {
        return 0;
    }
    if n == 2 {
        return 1;
    }
    return fib(n - 2) + fib(n - 1);
}

fn show_time(start_time: Instant) {
    let elapsed = start_time.elapsed();
    println!("実行時間: {:?}", elapsed);
}

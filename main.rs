use std::sync::mpsc;
use std::thread;

// Создается канал, в котором можно каждый поток вычисляет квадрат числа. Потом из Receiver'а мы получаем значения всех потоков и суммируем их.
fn main() {
    const n: usize = 10;
    let mut result: usize = 0;
    let mut massive: [usize; n] = [0; n];
    for i in 0..n {
        massive[i] = i + 2;
    }
    let (tx, rx) = mpsc::channel();

    let mut vector = Vec::new();
    for number in massive {
        let txi = tx.clone();
        let current_thread = thread::spawn(move || txi.send(number * number));
        vector.push(current_thread);
    }

    for handle in vector {
        handle.join().unwrap();
    }
    drop(tx);

    for answer in rx {
        result += answer;
    }

    print!("{}", result);
}
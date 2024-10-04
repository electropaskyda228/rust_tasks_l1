use std::thread;
use std::thread::JoinHandle;

// Создается тренировочный массив. Затем создается вектор потоков. Создаются потоки по выводу квадратного значения чисел исходного массива и записываются в массив.
// В конце вызываются итоги вычислений потоков методом join().
fn main() {
    const n: usize = 10;
    let mut massive: [usize; n] = [0; n];
    for i in 0..n {
        massive[i] = i + 2;
    }

    let mut vector: Vec<JoinHandle<()>> = Vec::new();
    for number in massive {
        let current_thread = thread::spawn(move || println!("{}", &(number * number)));
        vector.push(current_thread);
    }

    for handle in vector {
        handle.join().unwrap();
    }
}
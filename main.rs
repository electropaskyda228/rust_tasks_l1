use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main(){
    let cnt = Arc::new(Mutex::new(0));
    let mut vector = Vec::new();
    for _ in 0..10{
        let new_cnt = cnt.clone();
        let thread = thread::spawn(move || {
            let mut countr = new_cnt.lock().unwrap();
            *countr += 1;
        });
        vector.push(Option::Some(thread));
    }
    thread::sleep(Duration::from_secs(2));
    for handle in vector {
        match handle{
            Some(handle) => handle.join().unwrap(),
            None => continue
        }
    }
    println!("{}", cnt.lock().unwrap());
}
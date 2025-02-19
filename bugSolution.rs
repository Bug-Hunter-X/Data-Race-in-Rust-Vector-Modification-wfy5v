use std::sync::{Arc, Mutex};

fn main() {
    let v = Arc::new(Mutex::new(vec![1, 2, 3]));
    let v1 = v.clone();

    let thread1 = std::thread::spawn(move || {
        let mut v = v1.lock().unwrap();
        *v.get_mut(0).unwrap() = 4;
    });

    let thread2 = std::thread::spawn(move || {
        let v = v.lock().unwrap();
        println!("v: {:?}", *v);
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
} 
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            print!("[S{}]", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        print!("[M{}]", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
    println!();

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    handle.join().unwrap();
}

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::thread;

    #[test]
    #[should_panic]
    fn deadlock() {
        let (tx, rx) = mpsc::channel();
        let m = Arc::new(Mutex::new(5));
        {
            let m = Arc::clone(&m);
            let t = thread::spawn(move || {
                {
                    let mut num = m.lock().unwrap();
                    *num = 6;
                    let mut next = m.lock().unwrap(); // deadlock
                    *next = 7;
                }
                tx.send(m).unwrap();
            });
        }

        // this test will timeout after 1 seconds
        thread::sleep(std::time::Duration::from_secs(1));

        match rx.try_recv() {
            Ok(_) => assert_eq!(*m.lock().unwrap(), 7),
            Err(mpsc::TryRecvError::Empty) => panic!("deadlock"),
            Err(mpsc::TryRecvError::Disconnected) => panic!("disconnected"),
        }
    }

    #[test]
    fn deadlock_detected() {
        let (tx, rx) = mpsc::channel();
        let m = Arc::new(Mutex::new(5));
        {
            let m = Arc::clone(&m);
            let t = thread::spawn(move || {
                {
                    let mut num = m.lock().unwrap();
                    *num = 6;
                    let mut next = m.lock().unwrap(); // deadlock
                    *next = 7;
                }
                tx.send(m).unwrap();
            });
        }

        // this test will time-out after 1 seconds
        thread::sleep(std::time::Duration::from_secs(1));

        match rx.try_recv() {
            Ok(_) => assert_eq!(*m.lock().unwrap(), 7),
            Err(mpsc::TryRecvError::Empty) => {
                println!("deadlock detected");
                assert!(true);
            },
            Err(mpsc::TryRecvError::Disconnected) => panic!("disconnected"),
        }
    }
}

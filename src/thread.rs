#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::Duration;

    #[test]
    fn a() {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });

        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
    }

    #[test]
    fn b() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(move || {
            println!("Here is vector: {:?}", v);
        });
        handle.join().unwrap();
    }
}
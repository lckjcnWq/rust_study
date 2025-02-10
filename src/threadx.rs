use std::thread;
use std::time::Duration;

pub fn get_thread(){
    let handle = thread::spawn(||{
        for i in 1..10{
            println!("hi number {} from the first thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();
    for i in 1..5{
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    
}


use std::sync::Mutex;
use std::sync::Arc;

/**
 * 使用mutex在多个线程中共享状态
 */
#[test]
fn test_mutex_to_share_status(){
    let counter = Arc::new(Mutex::new(0));
    let mut handlers = vec![];

    for _ in 0..10 {
        let tc = counter.clone();
        let handler = std::thread::spawn(move || {
            let mut num = tc.lock().unwrap();
            *num += 1;
        });
        handlers.push(handler);
    }

    for elem in handlers {
        elem.join().unwrap();
    }

    println!("mutex result is {}", *counter.lock().unwrap());
}
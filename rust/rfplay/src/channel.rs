
use std::time::Duration;
use std::sync::mpsc;
use std::thread;

#[test]
fn test_mpsc_channel(){
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);

    thread::spawn(move || {
        let vals = vec![
            String::from("a"),
            String::from("b"),
            String::from("c"),
            String::from("d"),
            String::from("e"),
            String::from("f"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("h"),
            String::from("i"),
            String::from("j"),
            String::from("k"),
            String::from("l"),
            String::from("m"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(10));
        }
    });

    
    for i in rx {
        println!("receive {} from channel", i);
    }

}

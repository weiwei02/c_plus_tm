use std::thread;
use std::time::Duration;
use std::sync::mpsc;


fn main() {
    // print_fn();
    // closesure();
    join_thread();

    test_move();
    test_channel();
}

fn spawn_function(){
    for i in 0..5 {
        println!("spawn print {}", i);
        thread::sleep(Duration::from_millis(1));
    }
}

fn print_fn(){
    thread::spawn(spawn_function);
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep_ms(32);
    }
}

/**闭包语法
 * |参数1, 参数2, ...| -> 返回值类型 {
    // 函数体
}
*/
fn closesure(){
    thread::spawn(||{
        for i in 0..5 {
            println!("spawn thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep_ms(32);
    }
}

/**
 * join 方法
*/
fn join_thread(){
    let handler = thread::spawn(||{
        for i in 0..5 {
            println!("spawn thread print {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..3 {
        println!("main thread print {}", i);
        thread::sleep_ms(32);
    }
    handler.join().unwrap();
}

/**
 * 在子线程中尝试使用当前函数的资源，这一定是错误的！
 * 因为所有权机制禁止这种危险情况的产生，它将破坏所有
 * 权机制销毁资源的一定性。我们可以使用闭包的 move 关键字来处理：
*/
fn test_move(){
    let s = "hello";
   
    let handle = thread::spawn(move || {
        println!("move: spawn thread print {}", s);
    });

    handle.join().unwrap();
    println!("move: main thread print {}",s);
}

/**
 * Rust 中一个实现消息传递并发的主要工具是通道（channel），
 * 通道有两部分组成，一个发送者（transmitter）和一个接收者（receiver）。

std::sync::mpsc 包含了消息传递的方法
*/
fn test_channel() {
    let (transmitter,receiver) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hello");
        transmitter.send(val).unwrap();
    });
    let msg = receiver.recv().unwrap();
    println!("channel: main thread received {}", msg);
}
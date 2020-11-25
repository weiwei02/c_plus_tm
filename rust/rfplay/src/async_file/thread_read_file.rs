use std::sync::RwLock;
use lazy_static::lazy_static;
use std::thread;


use super::file;



 /**
  * 全局文件锁
  */
lazy_static!{
    static ref FILE1: RwLock<String> = RwLock::new(String::from(""));
    static ref FILE2: RwLock<String> = RwLock::new(String::from(""));
}


/**
 * 多线程读取文件测试
 */
fn test_read_file(){
    println!("program start!");

    let thread1 = thread::spawn(||{
        let mut w1 = FILE1.write().unwrap();
        *w1 = file::read_file("src/async_file/a.txt").unwrap();
        println!("read file 1");
    });

     let thread2 = thread::spawn(||{
        let mut w1 = FILE2.write().unwrap();
        *w1 = file::read_file("src/async_file/b.txt").unwrap();
        println!("read file 2");
    });
}

#[test]
fn test_file(){
    test_read_file();

    let mut rf1: bool = false;
    let mut rf2: bool = false;

    loop{
        let f1 = FILE1.read().unwrap();
        let f2 = FILE2.read().unwrap();

        if *f1 != String::from("") && !rf1 {
            println!("complete file read 1");
            rf1 = true;
        }
        if *f2 != String::from("") && !rf2 {
            println!("complete file read 2");
            rf2 = true;
        }
        
        if rf2 && rf1 {
            break;
        }
    }
}


#[test]
fn test_try_read(){
    test_read_file();

    let mut rf1: bool = false;
    let mut rf2: bool = false;

    loop{
        let f1 = FILE1.try_read();
        let f2 = FILE2.try_read();

        if let Ok(v) = f1 {
            if *v != String::from("") && rf1 == false {
                println!("completed file 1");
                rf1 = true;
            }
        }
        
        if let Ok(v) = f2 {
            if *v != String::from("") && rf2 == false {
                println!("completed file 2");
                rf2 = true;
            }
        }
        
        if rf2 && rf1 {
            break;
        }
    }
}
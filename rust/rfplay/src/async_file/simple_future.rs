/**
 * 使用阻塞模式的future
 * 与多线程类似，异步编程中也有一些陷阱和问题。事实是，async关键字不会神奇地使代码异步；
 * 它只是使函数返回Future。仍然必须繁重地安排代码执行时间。

* 这意味着函数必须迅速返回尚未准备就绪的状态，而不是被困在进行计算的过程中。
在我们的情况下，阻塞是特定在File::Open和file.read_to_string处发生的。这两个函数不是异步的，因此会阻止执行。
*/
use futures::{executor::block_on, join};

use super::file;

async fn load_file_1(){
    let rt1 =  file::async_read_file("src/async_file/a.txt").await;
    println!("file 1 size :{}", rt1.unwrap().len());
}

async fn load_file_2(){
    let rt1 =  file::async_read_file("src/async_file/b.txt").await;
    println!("file 2 size :{}", rt1.unwrap().len());
}

async fn load_files(){
    join!(load_file_1(), load_file_2());
}

#[test]
fn test_simple_future() {
    println!("program start");
    block_on(load_files());
    println!("program end");
}
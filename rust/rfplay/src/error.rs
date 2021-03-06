/**错误处理
 * Rust 有一套独特的处理异常情况的机制，它并不像其它语言中的 try 机制那样简单。

首先，程序中一般会出现两种错误：可恢复错误和不可恢复错误。

可恢复错误的典型案例是文件访问错误，如果访问一个文件失败，有可能是因为它正在被占用，是正常的，我们可以通过等待来解决。

但还有一种错误是由编程中无法解决的逻辑错误导致的，例如访问数组末尾以外的位置。

大多数编程语言不区分这两种错误，并用 Exception （异常）类来表示错误。在 Rust 中没有 Exception。

对于可恢复错误用 Result<T, E> 类来处理，对于不可恢复错误使用 panic! 宏来处理。
 */
fn main(){
    err();
    let_if_err();
}

fn panic1(){
    panic!("error occured");
    println!("Hello, Rust");
}


use std::fs::File;
fn err() {
    let f = File::open("hello.txt");
    match f {
        Ok(file) => {
            println!("File opened successfully.");
        },
        Err(e) => {
            println!("Failed to open the file. {}", e);
        }
    }
}

/**使用let if预发简化异常处理 */
fn let_if_err() {
    let f = File::open("hello.txt");
    if let Ok(file) = f{
        println!("File opened successfully.");
    }else{
        println!("Failed to open the file.");
    }
}

/**可恢复的错误的传递 */
fn f(i: i32) -> Result<i32, bool> {
    if i >= 0 { Ok(i) }
    else { Err(false) }
}
/**? 符的实际作用是将 Result 类非异常的值直接取出，如果有异常就将异常 Result 返回出去。
 * 所以，? 符仅用于返回值类型为 Result<T, E> 的函数，其中 E 类型必须和 ? 所处理的 
 * Result 的 E 类型一致。 */
fn g(i: i32) -> Result<i32, bool> {
    let t = f(i)?;
    Ok(t) // 因为确定 t 不是 Err, t 在这里已经是 i32 类型
}

/**kind 方法获取异常类型 */
fn read_text_from_file(path: &str) -> Result<String, io::Error> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
fn test_kind(){
    let str_file = read_text_from_file("hello.txt");
    match str_file {
        Ok(s) => println!("{}", s),
        Err(e) => {
            match e.kind() {
                io::ErrorKind::NotFound => {
                    println!("No such file");
                },
                _ => {
                    println!("Cannot read the file");
                }
            }
        }
    }
}
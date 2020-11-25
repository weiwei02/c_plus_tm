
use async_std::{fs::File, io};
use async_std::io::prelude::*;

/**
 * 使用异步标准库来读取文件
 * 
 * 这样读写文件时，就能使用正确的io调度
*/
async fn read_file_async( path : &str) -> io::Result<String>{
    let mut f = File::open(path).await?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer).await?;
    Ok(buffer)
    
}


/**枚举 */
fn main(){
    print_enum();
}

#[derive(Debug)]
enum Book {
    Papery {index: u32},
    Electronic {url: String},
}

fn print_enum(){
    let book = Book::Papery{index:12};
    println!("{:?}", book);
}

/**
 * 枚举的目的是对某一类事物的分类，分类的目的是为了对不同的情况进行描述。基于这个原理，往往枚举类最终都会被分支结构处理
 * （许多语言中的 switch ）。 switch 语法很经典，但在 Rust 中并不支持，很多语言摒弃 switch 的原因都是因为 switch 
 * 容易存在因忘记添加 break 而产生的串接运行问题，Java 和 C# 这类语言通过安全检查杜绝这种情况出现。
 */
fn match_enum(){
    let book = Book::Papery{index: 1001};
    let ebook = Book::Electronic{url: String::from("url...")};
   
    match book {
        Book::Papery { index } => {
            println!("Papery book {}", index);
        },
        Book::Electronic { url } => {
            println!("E-book {}", url);
        }
    }
}

/**Option 是 Rust 标准库中的枚举类，这个类用于填补 Rust 不支持 null 引用的空白。 */
fn option_enum(){
    let opt = Option::Some("Hello");
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}

/**option 必须声明时明确指定类型 */
fn option_type() {
    
    let opt: Option<&str> = Option::None;
    match opt {
        Option::Some(something) => {
            println!("{}", something);
        },
        Option::None => {
            println!("opt is nothing");
        }
    }
}
mod  structs;
pub fn eat_at_restaurant() {
    let mut meal = structs::back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

/**使用use关键字，将模块引入到当前作用域 */
use structs::back_of_house::Breakfast;
fn use_keyword() {
    let first = Breakfast::summer("Rye");
}


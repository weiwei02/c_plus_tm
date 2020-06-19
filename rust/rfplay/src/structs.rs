fn main(){
    instance_struct();
    tuple_struct();
    print_struct();
}

struct Site {
    domain: String,
    name:   String,
    nation: String,
    found:  u32
}

fn instance_struct(){
    let runobb = Site{
        domain: String::from("www.runoob.com"),
        name: String::from("RUNOOB"),
        nation: String::from("China"),
        found: 2013
    };
    println!("{}", runobb.name);
}

fn instance_struct2(){
    let domain = String::from("www.runoob.com");
    let name = String::from("RUNOOB");
    let runoob = Site {
        domain,  // 等同于 domain : domain,
        name,    // 等同于 name : name,
        nation: String::from("China"),
        found: 2013
    };
    println!("{}", (&runoob).name);
}

/** 
 * 有一种更简单的定义和使用结构体的方式：元组结构体。

元组结构体是一种形式是元组的结构体。

与元组的区别是它有名字和固定的类型格式。它存在的意义是为了处理那些需要定义类型（经常使用）又不想太复杂的简单数据：
 * 
 */
fn tuple_struct(){
    struct Color(u8, u8, u8);
    struct Point(f64, f64);

    let black = Color(0, 0, 0);
    let origin = Point(0.0, 0.0);

    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {})", origin.0, origin.1);
}


/**一定要导入调试库 #[derive(Debug)] ，之后在 println 和 print 宏中就可以用 {:?} 占位符输出一整个结构体： */
#[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
fn print_struct(){
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1 is {:?}", rect1);
}

impl Rectangle {
    /**结构体函数 */
    fn create(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }
    /**结构体方法 */
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn test_struct_method(){
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("rect1's area is {}", rect1.area());
}

pub fn struct_func(){
    let rect = Rectangle::create(30, 50);
    println!("{:?}", rect);
}

pub mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}
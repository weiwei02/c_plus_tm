/**
 * 所有权有以下三条规则：

* Rust 中的每个值都有一个变量，称为其所有者。
* 一次只能有一个所有者。
* 当所有者不在程序运行范围时，该值将被删除。
这三条规则是所有权概念的基础。
 */
fn sdsa(){
    // 在声明以前，变量 s 无效
    let s = "runoob";
    // 这里是变量 s 的可用范围
}
// 变量范围已经结束，变量 s 无效

fn main(){
    fn_param_owner();
}
/**
 * 涉及函数的所有权机制
对于变量来说这是最复杂的情况了。

如果将一个变量当作函数的参数传给其他函数，怎样安全的处理所有权呢？

下面这段程序描述了这种情况下所有权机制的运行原理：
 */
fn fn_param_owner(){
    // s 被声明有效
    let s = String::from("hello");

    // s 的值被当做参数传入函数，所以可以看做 s 已经被移动，从这里开始就变得无效
    takes_ownership(s);

    // x 被当做参数传入函数，但x是基本数值
    let x = 5;
    makes_copy(x);
    // 这里依然能够使用x，但不能使用s

    let s1 = gives_ownership();
    // gives_ownership 移动它的返回值到 s1

    let s2 = String::from("hello");
    // s2 被声明有效

    let s3 = takes_and_gives_back(s2);
    // s2 被当作参数移动, s3 获得返回值所有权

    // 引用对象不会转移所有权
    let len = borrowOwner(&s3);

    println!("s3 is {} ,len is {}", s3, len);

    reference_not_moddify();
    reference_moddify();
}

// 一个string有效参数传入
fn takes_ownership(somestring:String){
    println!("{}", somestring);
    // 函数结束，somestring 在这里释放
}

fn makes_copy(some_integer: i32) {
    // 一个 i32 参数 some_integer 传入，有效
    println!("{}", some_integer);
} // 函数结束, 参数 some_integer 是基本类型, 无需释放

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    // some_string 被声明有效

    return some_string;
    // some_string 被当作返回值移动出函数
}

fn takes_and_gives_back(a_string: String) -> String { 
    // a_string 被声明有效
    
    a_string  // a_string 被当作返回值移出函数
}

fn borrowOwner(s: &String) ->usize{
    s.len()
}

fn reference_not_moddify(){
    let s1 = String::from("run");
    let s2 = &s1;
    println!("{}", s2);
    // s2.push_str("oob"); // 错误，禁止修改租借的值
    println!("{}", s2);
}
fn reference_moddify(){
    let mut s1 = String::from("run");
    let s2 =  &mut s1;
    println!("{}", s2);
    s2.push_str("oob"); // 允许修改租借值
    println!("{}", s1);
}

/**
 * 可变引用与不可变引用相比除了权限不同以外，可变引用不允许多重引用，但不可变引用可以：
 * Rust 对可变引用的这种设计主要出于对并发状态下发生数据访问碰撞的考虑，在编译阶段就避免了这种事情的发生。
 * 由于发生数据访问碰撞的必要条件之一是数据被至少一个使用者写且同时被至少一个其他使用者读或写，所以在一个
 * 值被可变引用时不允许再次被任何引用。
 */
fn reference_repeat_moddify(){
    let mut s1 = String::from("run");
    let r1 = &mut s1;
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
}
fn reference_repeat_moddify2(){
    let mut s1 = String::from("run");
    let r1 = &mut s1;
    prin(r1);
    prin(r1);
    // let r2 = &mut s1;
    // println!("{}, {}", r1, r2);
}

fn prin(s :& mut String) {
    println!("{}", s);
}
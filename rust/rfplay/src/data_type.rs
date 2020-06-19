// 元组数据类型,用一对 ( ) 包括的一组数据，可以包含不同种类的数据
fn mul_type(){
    let tup:(i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("tuple y is {}", y);
}

// 使用[] 定义同数据类型的数组
fn array_define() {
    let a = [1,2,3,4,5];
    let b = ["January", "February", "March"];

    // c 是一个长度为5的i32数组
    let c:[i32; 5] = [1,2,3,4,5];

    // 等同于 [3,3,3,3,3]
    let d = [3; 5];
    
    // 数组访问
    let first = a[0];
    let second = a[1];

    // 错误，数组不可变
    // a[0] = 123;
    
    let mut a = [1,2,3];
    // 正确
    a[0] = 4;
    println!("array a[0] is {}",a[0]);

}

#[test]
fn test_data_type() {
    mul_type();
    array_define();
}
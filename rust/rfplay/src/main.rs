mod test_double;
fn main() {
    let a = 12;
    let a = a + 1;
    let a = a * 2;
    println!("Hello, world! a is {}", a);

    multable_variable();
}

fn multable_variable() {
    let mut s = "123";
    s = "i am a multable variable";
    println!("string is {}", s);
}
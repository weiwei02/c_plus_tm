/**
 * 泛型
*/
fn main() {}

fn max<T>(array: &[T]) -> T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i] > array[max_index] {
            max_index = i;
        }
        i += 1;
    }
    array[max_index]
}

/**特性 trait 类似于接口
 * Descriptive 规定了实现者必须有是 describe(&self) -> String 方法。
*/
trait Descriptive {
    fn describe(&self) -> String;
}

struct Person {
    name: String,
    age: u8,
}

impl Descriptive for Person {
    fn describe(&self) -> String {
        format!("{} {}", self.name, self.age)
    }
}

/**
 * 特性作为函数参数
*/
fn output(object: impl Descriptive) {
    println!("{}", object.describe());
}

/**
 * 特性泛型
*/
fn output_two<T: Descriptive>(arg1: T, arg2: T) {
    println!("{}", arg1.describe());
    println!("{}", arg2.describe());
}

/**多重继承*/
fn some_function<T: Display + Clone, U: Clone + Debug>(t: T, u: U) {}
/**简化写法，多重继承*/
fn some_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
}

trait Comparable {
    fn compare(&self, object: &Self) -> i8;
}

fn max<T: Comparable>(array: &[T]) -> &T {
    let mut max_index = 0;
    let mut i = 1;
    while i < array.len() {
        if array[i].compare(&array[max_index]) > 0 {
            max_index = i;
        }
        i += 1;
    }
    &array[max_index]
}

impl Comparable for f64 {
    fn compare(&self, object: &f64) -> i8 {
        if &self > &object {
            1
        } else if &self == &object {
            0
        } else {
            -1
        }
    }
}

fn compare_f64() {
    let arr = [1.0, 3.0, 5.0, 4.0, 2.0];
    println!("maximum of arr is {}", max(&arr));
}

/**特性作为返回值*/
fn person() -> impl Descriptive {
    Person {
        name: String::from("Cali"),
        age: 24
    }
}
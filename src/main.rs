fn main() {
    println!("Hello, world!");
}

#[test]
fn simple_test() {
    println!("Hello, world!");
}

fn addition(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn addition_test() {
    let result = addition(10, 20);
    assert_eq!(result, 30,"10 + 20 its should be 30");
}
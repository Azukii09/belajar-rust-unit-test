fn main() {
    println!("Hello, world!");
}

// A simple test to try unit testing.
#[test] //The mandatory attribute required for testing.
fn simple_test() {
    println!("Hello, world!");
}

// A function for adding numbers.
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

// A test for the number addition function using Rust's assertion macro.
#[test]
fn addition_test() {
    let result = addition(10, 20);
    assert_eq!(result, 30,"10 + 20 it's should be 30");
}

// A simple function that runs the application with checks.
fn start_application(host:&str,port:u16){
    if host == "localhost"{
        panic!("You must not use localhost");
    }else { 
        println!("Starting application in host {}:{}...",host,port);
    }
}

// A test for the simple function running an application that includes a panic.
#[test]
#[should_panic] //The test attribute that indicates that the tested function should panic.
fn starting_app_test() {
    start_application("localhost",8000);
}
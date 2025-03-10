fn main() {
    println!("Hello, world!");
}

// A simple test to try unit testing.
#[test] // Marks this function as a test function, so it will be executed when running `cargo test`.

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

#[test]
// Indicates that this test will be ignored by default when running `cargo test`.
// To run ignored tests, use `cargo test -- --ignored`.
#[ignore]
fn ignored_test() {
    println!("This test is ignored.");
}

#[test]
// Defines a test function named `addition_with_result_test`, which returns a `Result` type.
// If the test passes, it returns `Ok(())`, otherwise, it returns an `Err` with an error message.
fn addition_with_result_test()->Result<(),String> {
    let result = addition(10, 20);
    if result == 30 {
        // If the condition is met, the test passes by returning `Ok(())`.
        Ok(())
    }else {
        // If the condition is not met, the test fails by returning an error message as `Err`.
        Err("Addition result should be 30".to_string())
    }
}
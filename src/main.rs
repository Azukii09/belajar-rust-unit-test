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
    assert_eq!(result, 30,"10 + 20 it's should be 30");
}

fn start_application(host:&str,port:u16){
    if host == "localhost"{
        panic!("You must not use localhost");
    }else { 
        println!("Starting application in host {}:{}...",host,port);
    }
}

#[test]
#[should_panic]
fn starting_app_test() {
    start_application("localhost",8000);
}
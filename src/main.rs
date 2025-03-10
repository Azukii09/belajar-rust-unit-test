fn main() {
    println!("Hello, world!");
}

// simple test untuk mencoba unit test
#[test] //atribut wajib yang diperlukan untuk melakukan pengujian
fn simple_test() {
    println!("Hello, world!");
}

// fungsi penambahan angka
fn addition(a: i32, b: i32) -> i32 {
    a + b
}

// test untuk fungsi penambahan angka menggunakan macro assertion dari rust
#[test]
fn addition_test() {
    let result = addition(10, 20);
    assert_eq!(result, 30,"10 + 20 it's should be 30");
}

// fungsi simpel menjalankan aplikasi untuk pengecekan
fn start_application(host:&str,port:u16){
    if host == "localhost"{
        panic!("You must not use localhost");
    }else { 
        println!("Starting application in host {}:{}...",host,port);
    }
}

// pengujian fungsi simpel menjalankan aplikasi yang mengandung panic
#[test]
#[should_panic] //atribut pengujian yang menyatakan bahwa fungsi yang dites harus terjadi panic
fn starting_app_test() {
    start_application("localhost",8000);
}
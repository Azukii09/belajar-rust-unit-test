// Defines a public function `hello_name` that takes a string slice (`&str`) as an argument.
// Returns a formatted `String` that includes the given name.
pub fn hello_name(name: &str) -> String {
    // Uses `format!` to create a greeting message by inserting the `name` into the string.
    format!("Hello, {}!", name)
}

// The following module is only compiled and included when running tests (`cargo test`).
#[cfg(test)]
mod tests {
    // Imports the `hello_name` function from the parent module to be used in tests.
    use super::hello_name;

    // Marks this function as a test, so it will be executed when running `cargo test`.
    #[test]
    // Defines a test function named `hello_test`.
    fn hello_test(){
        // Calls the `hello_name` function with `"gege"` as an argument and stores the result.
        let result = hello_name("gege");
        // Asserts that the result is `"Hello, gege!"`. If it is not, the test fails.
        assert_eq!(result, "Hello, gege!");
    }
}
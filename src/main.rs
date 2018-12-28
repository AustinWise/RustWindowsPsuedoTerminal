fn main() {
    let mut hello_str : &str = "Hello cruel world";
    println!("{}", hello_str);
    hello_str = "Hello borrow-checking world!";
    println!("{}", hello_str);
}

fn main() {
    println!("x + 5 = {}", another_function(5));

}

// Here is a comment!
fn another_function(x: i32) -> i32 {
    println!("The value of x is: {x}");
    x + 5
}

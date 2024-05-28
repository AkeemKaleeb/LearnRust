use std::io;

fn main() {
    let sume = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5/3;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;

    let c = 'z';
    let z: char = 'Z';
    let heart_eyed_cat = '😻';

    let tup: (i32, f64, u8) = (500, 6.3, 1);
    let (x, y, z) = tup;
    println!("The value of y is: {y}");

    let five_hundred = tup.0;

    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", 
        "July", "August", "September", "October", "November", "December"];
    let b: [i32; 5] = [1, 2, 3, 4, 5];
    let first = b[0];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to read line");

    let index: usize = index
    .trim()
    .parse()
    .expect("Index entered was not a number");

    let element = a[index];


    println!("The value of the element at index {index} is: {element}");

}

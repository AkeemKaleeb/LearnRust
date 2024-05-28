fn main() {
    /* let number = 8;

    if number < 5 {
        println!("Number was less than 5");
    }
    else if number < 10 {
        println!("Number was between 5 and 10");
    }
    else {
        println!("Number was more than 10");
    }

    let condition = true;
    let _num = if condition { 5 } else { 6 };

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("The value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!!"); */

    println!("{} degrees farenheit equals {} degrees celsius", 32, farenheit_to_celsius(32.0));
    println!("{} degrees farenheit equals {} degrees celsius", 100, farenheit_to_celsius(100.0));
    println!("{} degrees celsius equals {} degrees farenheit", 0, celsius_to_farenheit(0.0));
    println!("{} degrees celsius equals {} degrees farenheit", 100, celsius_to_farenheit(100.0));

    println!("The {} term of the fibonacci sequence is {}", 1, fibonacci(1));
}

fn farenheit_to_celsius(temperature:f64) -> f64{
    (temperature - 32.0) * 5.0 / 9.0
}

fn celsius_to_farenheit(temperature:f64) -> f64{
    (temperature * 9.0 / 5.0) + 32.0
}

fn fibonacci(term:i32) -> i32 {
    let mut count = 1;
    let mut last_fib = 1;
    let mut fib = 0;

    while count < term {
        let temp = fib;
        fib += last_fib;
        last_fib = temp;
        count += 1;
    }

    return fib;
}

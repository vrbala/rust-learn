
fn fib(n: u32) -> u32 {
    if n < 2 {
        n
    } else {
        fib(n-2) + fib(n-1)
    }
}

fn main() {

    // if is a expression - so it returns value
    // similar to pattern matching, different branches of an if expression are called arms

    let number = 3;

    if number < 5 {
        println!("less than 5");
    } else {
        println!("greater than 5");
    }

    // since if is an expression -
    // !!! obviously, both arms should evaluate to same type
    let _another = if number < 5 {
        3
    } else {
        7
    };

    // loop is an expression
    // thus, can be used for retry operations
    let _x = loop {
        break 5; // makes the loop to evaluate to 5 
    };

    // while - entry controlled loop
    let mut counter = 3;
    while counter > 0 {
        counter -= 1;
    }
    println!("Countdown complete ");

    // for loop - useful for iterations
    let a = [1, 2, 3, 4, 5];
    println!("array: {:?}", a);

    for y in a.iter() { // .iter/.rev = iterator/reverse iterator
        println!("Current: {}", y);
    }

    for y in (1..5).rev() {
        println!("Current: {}", y);
    }

    println!("Enter a number: ");
    let mut number = String::new();
    std::io::stdin().read_line(&mut number)
        .expect("Error reading input.");
    let number : u32 = number.trim().parse()
        .expect("Invalid number.");

    println!("{}th fibonacci's number is: {}", number, fib(number));
}

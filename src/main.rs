//Rust calculator: Simple shit but... you know: Mwahahaha

// Todo: Seperate functions to handle calculations

fn addition(a: i32, b: i32) -> i32 {
    println!("Addition");
    return a + b;
}

fn subtraction(a: i32, b: i32) -> i32 {
    println!("Subtraction");
    return a - b;
}

fn multiplication(a: i32, b: i32) -> i32 {
    println!("Multiplication");
    return a * b;
}

fn division(a: i32, b: i32) -> i32 {
    println!("Division");
    return a / b;
}

fn modulus(a: i32, b: i32) -> i32 {
    println!("Modulo");
    return a % b;
}

fn main() {
    let choice = 6;
    let a = 200;
    let b = 30;

    println!("Simple Rust Calculator: Wahahaha");
    println!("1. Addition\n2. Subtraction\n3. Multiplication\n4. Division\n5. Modulo\n");

    println!("The selected choice is {choice}");
    match choice {
        1 => {
            println!("Result: {}", addition(a, b));
        }
        2 => {
            println!("Result: {}", subtraction(a, b));
        }
        3 => {
            println!("Result: {}", multiplication(a, b));
        }
        4 => {
            println!("Result: {}", division(a, b));
        }
        5 => {
            println!("Result: {}", modulus(a, b));
        }
        _ => println!("Invalid Output brooo! Check the friggin options"),
    }
}

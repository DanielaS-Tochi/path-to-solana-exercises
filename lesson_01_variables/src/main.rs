// fn main() {
//     println!("Hello, world!");
// }
// This is how you can write a simple Rust program that prints "Hello, world!" to the console.
// The main function is the entry point of the program, and println! is a macro that
// prints text to the console. The exclamation mark indicates that println! is a macro,
// not a function. You can run this code in a Rust environment to see the output. This is how main.rs might look like in a Rust project and its code when the project is created for the first time.

mod exercises; 

fn main() {
    // Immutable variable
    let name = "Dani";
    println!("Hello, {}!", name);

    // Mutable variable
    let mut age = 30;
    println!("Current age: {}", age);
    age += 1;
    println!("Next year, you'll be {}.", age);

    // Constants
    const SECONDS_IN_MINUTE: u32 = 60;
    println!("There are {} seconds in a minute.", SECONDS_IN_MINUTE);

    // Shadowing
    let score = 10;
    let score = score + 5;
    println!("Your score is: {}", score);

    // Type annotation
    let pi: f64 = 3.14159;
    println!("Pi is approximately {}", pi);

     // Ejecutar ejercicios
    println!("\n📘 Running exercises:");
    exercises::run();
}
// This is a simple Rust program demonstrating variables, mutability, constants, shadowing, and type annotations.
// It showcases how to declare and use variables, both mutable and immutable, as well as constants
// and how to shadow variables in Rust. The program also includes type annotations for clarity.
// The main function serves as the entry point of the program, printing messages to the console.
// The code is structured to be clear and educational, making it suitable for beginners learning Rust.
// The program is designed to be run in a Rust environment, and it will output the results
// of the variable manipulations to the console.
// The code is written in a straightforward manner, focusing on basic concepts of Rust programming.
// The main function is the starting point of the program, where it executes the defined logic.
// The program is intended to be simple and easy to understand, making it a good example for
// those new to Rust or programming in general. It highlights the key features of Rust's variable
// system, including immutability, mutability, constants, shadowing, and type annotations.
// The output will demonstrate the use of these features in a practical context, providing a hands-on
// experience for learners. The code is concise and to the point, ensuring that it remains accessible
// to those who may not have prior programming experience. The use of comments helps clarify the purpose
// of each section, making it easier for readers to follow along and understand the concepts being presented.   
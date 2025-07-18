pub fn run() {
    println!("\n-- Practice Exercises --");

    // 🔸 Exercise 1: Create a mutable variable called 'counter' and set it to 0.
    // Then increment it by 1 and print the result.
    let mut counter = 0;
    counter += 1;
    println!("Counter is now: {}", counter);

    // 🔸 Exercise 2: Declare a constant named MAX_POINTS with value 100.
    const MAX_POINTS: u32 = 100;
    println!("Max points: {}", MAX_POINTS);

    // 🔸 Exercise 3: Shadow a variable
    let value = 5;
    let value = value * 2;
    println!("Shadowed value: {}", value);

    // 🔸 Challenge: Create a variable `temperature_celsius` as a float,
    // and print it converted to Fahrenheit.
    let temperature_celsius: f64 = 20.0;
    let temperature_fahrenheit = (temperature_celsius * 9.0 / 5.0) + 32.0;
    println!(
        "{}°C is {}°F",
        temperature_celsius, temperature_fahrenheit
    );
}

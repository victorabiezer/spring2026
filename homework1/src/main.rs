// declare const 
const FREEZING_POINT_F: f64 = 32.0;

// fahrenheit --> celsius
fn fahrenheit_to_celsius(f: f64) -> f64 
{
    (f - FREEZING_POINT_F) * 5.0 / 9.0
    // no semicolon!!! the professor said it's an expression so returns value 
}

// celsius --> fahrenheit  
fn celsius_to_fahrenheit(c: f64) -> f64 
{
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}


// assignment 2 

// check if number == even
fn is_even(n: i32) -> bool 
{
    n % 2 == 0
}


// assignment 3
// check guess against secret
fn check_guess(guess: i32, secret: i32) -> i32 
{
    if guess == secret 
    {
        0
    } else if guess > secret 
    {
        1
    } else {
        -1
    }
}

// main 
fn main() 
{
    // assignment 1
    let mut temp_f: f64 = 32.0;
    let temp_c = fahrenheit_to_celsius(temp_f);
    println!("{}°F = {:.2}°C", temp_f, temp_c);
    
    for _ in 0..5 
    {
        temp_f += 1.0;
        let temp_c = fahrenheit_to_celsius(temp_f);
        println!("{}°F = {:.2}°C", temp_f, temp_c);
    }
    
    // assignment 2
    let numbers: [i32; 10] = [1, 2, 3, 4, 5, 6, 9, 10, 15, 20];
    
    for num in numbers 
    {
        if num % 3 == 0 && num % 5 == 0 
        {
            println!("FizzBuzz");
        } else if num % 3 == 0 
        {
            println!("Fizz");
        } else if num % 5 == 0 
        {
            println!("Buzz");
        } else if is_even(num) 
        {
            println!("Even");
        } else {
            println!("Odd");
        }
    }
    
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() 
    {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum: {}", sum);
    
    let mut largest = numbers[0];
    let mut i = 1;
    loop 
    {
        if i >= numbers.len() 
        {
            break;
        }
        if numbers[i] > largest 
        {
            largest = numbers[i];
        }
        i += 1;
    }
    println!("Largest: {}", largest);
    
    // assignment 3
    let secret = 42;
    let guesses = [30, 50, 40, 43, 42];
    let mut attempts = 0;
    
    for guess in guesses 
    {
        attempts += 1;
        let result = check_guess(guess, secret);
        
        if result == 0 
        {
            println!("coorrect!");
            break;
        } else if result == 1 {
            println!("too high!");
        } else {
            println!("too low...");
        }
    }
    
    println!("took {} guesses", attempts);
}
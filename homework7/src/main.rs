// task 1: basic closure
// concept: Closures for arithmetic operations
// closures are anonymous functions that can capture variables from their environment. they're useful for compact, inline logic.

// task 1: write a closure named operation that multiplies two integers and returns the result. Test it with 10 * 5 and print the result.

// fn main() 
// {
//     let operation = |a: i32, b: i32| 
//     {
//         // Your implementation here
//         a * b 
//     };

//     println!("Result: {}", operation(10, 5));
// }

// task 2: environment capture
// concept: capturing and modifying state
// closures can capture and modify variables from their environment, allowing state to persist between calls.

// task 2: write a closure named update inside a function track_changes. the closure should increment and print a counter each time it is called.

// fn track_changes() 
// {
//     let mut tracker = 0;
//     let mut update = || 
//     {
//         // Your implementation here
//         tracker += 1; 
//         println!("Counter: {}", tracker); 
//     };

//     update();
//     update();
// }

// fn main() 
// {
//     track_changes();
// }


// task 3: vector transformation
// concept: applying closures to transform vectors
// closures can be passed as arguments to functions for transforming elements in a vector.

// task 3: write a function process_vector that applies a closure to transform each element of a vector. implement it in both ways:
// 1. using map and collect
// 2. using a for loop

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     vec.into_iter().map(f).collect()
// }

// fn main() {
//     let numbers = vec![1, 2, 3];

//     let doubled = process_vector(numbers.clone(), |x| 
//     {
//         x * 2
//     });

//     let replaced = process_vector(numbers, |x| 
//     {
//         if x > 2 { 0 } else { x }
//     });

//     println!("Doubled: {:?}", doubled);
//     println!("Replaced: {:?}", replaced);
// }

// fn process_vector<F>(vec: Vec<i32>, f: F) -> Vec<i32>
// where
//     F: Fn(i32) -> i32,
// {
//     let mut result = Vec::new();
//     for x in vec {
//         result.push(f(x));
//     }
//     result
// }


// task 3 

use std::{thread, time::Duration};

struct ComputeCache<T>
where
    T: Fn() -> String,
{
    computation: T,
    value: Option<String>,
}

impl<T> ComputeCache<T>
where
    T: Fn() -> String,
{
    fn new(computation: T) -> Self 
    {
        ComputeCache 
        {
            computation,
            value: None,  // nothing cached yet
        }
    }

    fn get_result(&mut self) -> String {
        match self.value {
            Some(ref v) => {
                println!("Retrieved from cache instantly!");
                v.clone()
            }
            None => {
                let result = (self.computation)();
                self.value = Some(result.clone());
                result
            }
        }
    }
}

fn main() {
    let mut cache = ComputeCache::new(|| {
        println!("Computing (this will take 2 seconds)...");
        thread::sleep(Duration::from_secs(2));
        "Hello, world!".to_string()
    });

    println!("First call:");
    println!("Result: {}", cache.get_result());

    println!("\nSecond call:");
    println!("Result (cached): {}", cache.get_result());
}
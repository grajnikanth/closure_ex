// Closure functions - how they work and when they are used

use std::thread;
use std::time::Duration;

// The below function will be converted to a closure function in this example
fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("Calculating slowly");
    // thread::sleep function puts the thread to sleep for seconds specified. 
    // This will block the code for the specified number of seconds at this line of code

    thread::sleep(Duration::from_secs(2));
    intensity
}

fn main() {
    let intensity_per_user = 10;
    let random_number_hardcoded = 7;

    // call the function below to obtain a workout plan for the user based
    // on user input. User input is currently being hard coded
    // The logis is that below function would call the simla...() function
    // to obtain some parts of the workout plan
    generate_workout(intensity_per_user, random_number_hardcoded);
}

fn generate_workout(intensity: u32, random_number: u32) {

    // Code refactored to call the function simula.. only once at the begining here
    // expnesive_.. value is used in the if loop at three locations
    // it is the same value so we can make the code faster by calling only once
    // but we end up calling simu.. function even when it is not needed 
    // that is when the logic is such that intensity >= 25 and rando.. = 3
    // closures can solve this problem of only calling this function once and only
    // when needed. That will be implemented in the next git commit
    let expensive_result = simulated_expensive_calculation(intensity);
    
    
    // Logic to check various conditions to provide user a workout plan based
    // on user feedback

    if intensity < 25 {
        println!("Today, do {} pushups!", 
            expensive_result);
        println!("Next, do {} situps", 
            expensive_result);
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", 
                    expensive_result);
        }
    }
}



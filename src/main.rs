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

    // Logic to check various conditions to provide user a workout plan based
    // on user feedback

    if intensity < 25 {
        println!("Today, do {} pushups!", 
            simulated_expensive_calculation(intensity));
        println!("Next, do {} situps", 
            simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", 
                    simulated_expensive_calculation(intensity));
        }
    }
}



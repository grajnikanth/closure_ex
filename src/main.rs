// Closure functions - how they work and when they are used

use std::thread;
use std::time::Duration;

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

    // expensive_closure is a variable which stores and an anonymous function
    // the arguments and return value types of closures are interpreted by the 
    // compiler the first time a closure is called. The second time a closure is called
    // you cannot send in a different type as that would give a compiler error because
    // compiler locks in the types of the variables the first time closure is called
    // syntax and functionality similar to that of Javascript for anonymous functions

    // The below anonymous function eliminates the need for the simul... function
    // defined in the previous git commits
    let expensive_closure = |num| {
        println!("Calculating slowly");
        // thread::sleep function puts the thread to sleep for seconds specified. 
        // This will block the code for the specified number of seconds at this line of code
    
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    // Logic to check various conditions to provide user a workout plan based
    // on user feedback

    // We are still calling the same closure function three times. 
    // Next we will implement a struct to eliminate this calling of same anonymous
    // function thrice and also call it only when needed 
    if intensity < 25 {
        println!("Today, do {} pushups!", 
            expensive_closure(intensity));
        println!("Next, do {} situps", 
            expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", 
                    expensive_closure(intensity));
        }
    }
}



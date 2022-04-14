// Closure functions - how they work and when they are used

use std::thread;
use std::time::Duration;

// Implementation of struct called Cacher to store the closure
// value will store the value from the calculation closure function if value
// is None
// Generic T represents the closure function which should of type Fn(u32) -> u32
// per the Trait Bound definition
// Struct needs to know the type of fields. So we have to specify that our 
// calculation field will hold a closure function type Fn(u32) -> u32. That
// it takes a u32 argument and returns a u32 value
struct Cacher<T> 
where
    // Fn is one of the trait types closures implement. FnOnce and FnMut are the other
    // trait types closure implement
    T: Fn(u32) -> u32,
{
    // Both fields are private so they cannot be modified directly by code
    // They can only be modified via the methods on this struct
    calculation: T,
    value: Option<u32>,
}

// Note the syntax for the impl<T>

impl<T> Cacher<T> 
// Appears that the T trait bounds have to be defined even on this impl 
// even though it was defined on the Cacher Struct above. I guess this is so that
// different type Generic T could be used in the implemented function other that
// defined above with Struct cacher. Here T in both places happes to be the same
where 
    T: Fn(u32) -> u32
{
    // Return a new Cacher instance with value set to None and closure function
    // set to the calculation field
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // note the &mut self as the self will be changed in this function
    // we also want to return a value when this function is called a u32

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            // if self.value is None, we call the closure function stored in the
            // calculation field and send that closure function the arg value as
            // parameter
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}



fn main() {
    let intensity_per_user = 35;
    let random_number_hardcoded = 3;

    // call the function below to obtain a workout plan for the user based
    // on user input. User input is currently being hard coded
    // The logis is that below function would call the simla...() function
    // to obtain some parts of the workout plan
    generate_workout(intensity_per_user, random_number_hardcoded);
}

fn generate_workout(intensity: u32, random_number: u32) {

    // use of struct Cacher to call the closure only once and store the value for
    // future use in the struct

    // create an instance of the Cacher struct with new function and pass the closure 
    // function

    // mutable because theis cacher instance fields will change 
    // new returns a Cacher instance with the closure function now stored 
    // as a field calculation of this instance
    let mut expensive_result = Cacher::new(|num| {
        println!("Calculating slowly");
        // thread::sleep function puts the thread to sleep for seconds specified. 
        // This will block the code for the specified number of seconds at this line of code
    
        thread::sleep(Duration::from_secs(2));
        num
    });
    
    // Logic to check various conditions to provide user a workout plan based
    // on user feedback

   // now the .value() function called on the Cacher instance will only execute
   // the closure function only once and rest of the times it is going to 
   // retrieve the value stored in the Value field
   // closure will not be called at all if intensity > 25 and random number == 3
    if intensity < 25 {
        println!("Today, do {} pushups!", 
            expensive_result.value(intensity));
        println!("Next, do {} situps", 
            expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", 
                    expensive_result.value(intensity));
        }
    }
}



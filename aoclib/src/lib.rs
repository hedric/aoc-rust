use std::fs::read_to_string;
use std::time::Instant;

// Generic function that can measure execution time for any closure or function.
// F: the type of the closure or function
// R: return type of that closure or function
pub fn measure_time<F, R>(name: &str, f: F) -> R

// This where clause constrains F to be a closure or function that can be called exactly once
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    println!("{} took {:.3?}", name, duration);
    result
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .expect("unable to open file!")
        .lines() // split the string into an iterator of string slices
        .map(String::from) // make each slice into a string
        .collect() // gather them together into a vector
}


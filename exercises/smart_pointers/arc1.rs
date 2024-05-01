// arc1.rs
//
// In this exercise, we are given a Vec of u32 called "numbers" with values
// ranging from 0 to 99 -- [ 0, 1, 2, ..., 98, 99 ] We would like to use this
// set of numbers within 8 different threads simultaneously. Each thread is
// going to get the sum of every eighth value, with an offset.
//
// The first thread (offset 0), will sum 0, 8, 16, ...
// The second thread (offset 1), will sum 1, 9, 17, ...
// The third thread (offset 2), will sum 2, 10, 18, ...
// ...
// The eighth thread (offset 7), will sum 7, 15, 23, ...
//
// Because we are using threads, our values need to be thread-safe.  Therefore,
// we are using Arc.  We need to make a change in each of the two TODOs.
//
// Make this code compile by filling in a value for `shared_numbers` where the
// first TODO comment is, and create an initial binding for `child_numbers`
// where the second TODO comment is. Try not to create any copies of the
// `numbers` Vec!
//
// Execute `rustlings hint arc1` or use the `hint` watch subcommand for a hint.


#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::Arc;
use std::thread;

fn main() {
    let numbers: Vec<u32> = (0..100).collect();
    let shared_numbers = Arc::new(numbers); // TODO: Change this line

    let mut threads = vec![];

    for offset in 0..8 {
        let child_numbers = shared_numbers.clone(); // TODO: Change this line

        let thread = thread::spawn(move || {
            let mut sum = 0;

            for i in (offset..100).step_by(8) {
                sum += child_numbers[i];
            }

            sum
        });

        threads.push(thread);
    }

    let mut sum = 0;

    for thread in threads {
        sum += thread.join().unwrap();
    }

    println!("Sum: {}", sum);
}


use std::{thread, time};
use std::time::{Duration, Instant};

pub fn get_execution_time(step: u32, iterations: u32) -> f32 {

    //generate array of times
    let mut iter_durations: Vec<u32> = vec![];

    for i in 0..iterations {

        let start_time = Instant::now();
        fib(step);
        let duration = start_time.elapsed();

        let ns = duration.subsec_nanos();

        iter_durations.push(ns);
    }

    //get average value
    let total: u32 = iter_durations.iter().sum();
    let average = total as f32 / 1000.0;
    return average;
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fib(n - 1) + fib(n - 2), // O(2ⁿ)
    }
}
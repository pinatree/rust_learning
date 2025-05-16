use std::{thread, time};
use std::time::{Duration, Instant};

pub fn get_execution_time(items_count: usize, iterations: usize) -> f32 {

    let mut rng = rand::thread_rng();

    //generate random array of integers for search values
    let mut random_numbers: Vec<u32> = vec![];

    for i in 0..items_count {
        random_numbers.push(i as u32);
    }
    
    //generate array of durations
    let mut iter_durations: Vec<u32> = vec![];

    for i in 0..iterations {

        //get random value in range
        let search_value = rand::Rng::gen_range(&mut rng, 0.. items_count) as u32;

        let start_time = Instant::now();
        o_log_complexity_iteration(&mut random_numbers, search_value);
        let duration = start_time.elapsed();

        let ns = duration.subsec_nanos();

        iter_durations.push(ns);
    }

    //get average value
    let total: u32 = iter_durations.iter().sum();
    let average = total as f32 / 1000.0;
    return average;
}

fn o_log_complexity_iteration(arr: &mut Vec<u32>, id: u32) {
    //unsorted search
    for i in arr.iter() {
        if i == &id {
            return;
        }
    }
}
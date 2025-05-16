use std::{thread, time};
use std::time::{Duration, Instant};

pub fn get_execution_time(items_count: usize, iterations: usize) -> f32 {

    let mut rng = rand::thread_rng();

    //generate random array of integers for search values
    let mut random_numbers: Vec<u32> = vec![];

    for i in 0..items_count {
        let append = rand::Rng::gen_range(&mut rng, 0..100);
        random_numbers.push(append);
    }

    //generate random array of indexes
    let mut random_ids: Vec<usize> = vec![];

    for i in 0..iterations {
        let random_id = rand::Rng::gen_range(&mut rng, 0.. items_count);
        random_ids.push(random_id);
    }

    //generate array of times
    let mut iter_durations: Vec<u32> = vec![];

    for i in 0..iterations {
        let current_search_index = random_ids[i];

        let start_time = Instant::now();
        o_1_constant_time_iteration(&mut random_numbers, current_search_index);
        let duration = start_time.elapsed();

        let ns = duration.subsec_nanos();

        iter_durations.push(ns);
    }

    //get average value
    let total: u32 = iter_durations.iter().sum();
    let average = total as f32 / 1000.0;
    return average;
}

fn o_1_constant_time_iteration(arr: &mut Vec<u32>, id: usize) {
    let found = arr[id];
}
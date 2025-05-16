use rand::thread_rng;
use std::time::{Duration, Instant};

fn main() {
    println!("Разработка и отладка примеров сложности Big_O");

    o_1_constant_time();
}

fn o_1_constant_time() {

    let mut rng = rand::thread_rng();

    //generate random array of integers
    let mut random_numbers: [u32; 1000] = [0; 1000];

    for i in 0..1000 {
        random_numbers[i] = rand::Rng::gen_range(&mut rng, 0..100);
    }

    //generate random array of ids
    let mut random_ids: [usize; 100] = [0; 100];

    //generate random array of times
    let mut iter_durations: [u32; 100]= [0; 100];

    for i in 0..100 {
        random_ids[i] = rand::Rng::gen_range(&mut rng, 0..100);
    }

    for i in 0..100 {
        let current_index = random_ids[i];

        let start_time = Instant::now();
        o_1_constant_time_iteration(random_numbers, current_index);
        let duration = start_time.elapsed();

        let ns = duration.subsec_nanos();

        iter_durations[i] = ns;
    }

    //get average value
    let total: u32 = iter_durations.iter().sum();
    let average = total as f32 / 100.0;
    println!("Константная сложность, среднее время выполнения по 100 записям o(1) = {} наносекунд", average)
}

fn o_1_constant_time_iteration(arr: [u32; 1000], id: usize) {
    let found = arr[id];
}
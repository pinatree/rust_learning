use std::{thread, time};
use std::time::{Duration, Instant};

pub fn get_execution_time(step: u32, iterations: u32) -> f32 {

    //generate array of times
    let mut iter_durations: Vec<u32> = vec![];

    for i in 0..iterations {

        let start_time = Instant::now();
        factorial_complexity(step);
        let duration = start_time.elapsed();

        let ns = duration.subsec_nanos();

        iter_durations.push(ns);
    }

    //get average value
    let total: u32 = iter_durations.iter().sum();
    let average = total as f32 / 1000.0;
    return average;
}

fn factorial_complexity(n: u32) -> Vec<Vec<u32>> {
    // Базовый случай: если n = 0, возвращаем пустую перестановку
    if n == 0 {
        return vec![vec![]];
    }

    // Рекурсивно генерируем перестановки для n-1
    let mut permutations = factorial_complexity(n - 1);
    let mut result = Vec::new();

    // Вставляем число `n` во все возможные позиции каждой перестановки
    for perm in &permutations {
        for i in 0..=perm.len() {
            let mut new_perm = perm.clone();
            new_perm.insert(i, n);
            result.push(new_perm);
        }
    }

    result
}
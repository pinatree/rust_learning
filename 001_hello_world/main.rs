use std::{thread, time};

fn main() {
    println!("Hello world from RUST!");
    count_down();
    fly_rocket();
}

fn count_down() {
    let mut counter: u8;
    let final_value: u8;
    counter = 5;
    final_value = 1;

    loop {
        let ten_millis = time::Duration::from_millis(250);
        thread::sleep(ten_millis);

        counter = counter - 1;
        if counter < final_value {
            break;
        }
        println!("Rocket start after { } seconds", counter);
    }

}

fn fly_rocket() {    
    for _ in 1..15 {
        let output: &str;
        output = "WZWZWZWWZWZWZWZWZ!";

        println!("{}", output);

        let ten_millis = time::Duration::from_millis(100);
        thread::sleep(ten_millis);
    }
    println!("Rocket flew away!");
}
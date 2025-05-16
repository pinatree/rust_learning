mod constant_complexity;
mod logariphmic_complexity;
mod linear_complexity;
mod o_n_log_n_complexity;
mod square_complexity;
mod cube_complexity;
mod complexity_2_in_n;
mod factorial_complexity;

fn main() {
    println!("Разработка и отладка примеров сложности Big_O");

    bigo_complexity_o_1();
    bigo_complexity_o_log();
    bigo_complexity_linear();
    bigo_complexity_n_log_n();
    bigo_complexity_n_square_2();
    bigo_complexity_n_square_3();
    bigo_complexity_2_square_n();
    factorial_complexity();
}

fn bigo_complexity_o_1() {
    let mut counter = 100;
    let step = 100;
    let max = 10000;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = constant_complexity::get_execution_time(counter, 1000);

        println!("Сложность O(1), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_o_log() {
    let mut counter = 100;
    let step = 100;
    let max = 10000;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = logariphmic_complexity::get_execution_time(counter, 100);

        println!("Сложность Olog, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_linear() {
    let mut counter = 100;
    let step = 100;
    let max = 10000;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = linear_complexity::get_execution_time(counter, 100);

        println!("Сложность линейная - O(n), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_n_log_n() {
    let mut counter = 100;
    let step = 100;
    let max = 10000;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = o_n_log_n_complexity::get_execution_time(counter, 100);

        println!("Сложность линейнлогарфмическая - O(n log n), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_n_square_2() {
    let mut counter = 10;
    let step = 10;
    let max = 100;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = square_complexity::get_execution_time(counter, 1);

        println!("Сложность квадратичная - O(n^2), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_n_square_3() {
    let mut counter = 10;
    let step = 10;
    let max = 100;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = cube_complexity::get_execution_time(counter, 10);

        println!("Сложность кубическая - O(n^3), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn bigo_complexity_2_square_n() {
    let mut counter = 1;
    let step = 1;
    let max = 25;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = complexity_2_in_n::get_execution_time(counter, 10);

        println!("Сложность экспоненциальная, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}

fn factorial_complexity() {
    let mut counter = 1;
    let step = 1;
    let max = 8;

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = factorial_complexity::get_execution_time(counter, 10);

        println!("Сложность факториальная, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        counter = counter + step;
    }
}
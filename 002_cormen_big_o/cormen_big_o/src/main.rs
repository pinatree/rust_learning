mod constant_complexity;
mod logariphmic_complexity;
mod linear_complexity;
mod o_n_log_n_complexity;
mod square_complexity;

fn main() {
    println!("Разработка и отладка примеров сложности Big_O");

    bigo_complexity_o_1();
    bigo_complexity_o_log();
    bigo_complexity_linear();
    bigo_complexity_n_log_n();
    bigo_complexity_n_square_2();
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
        
        let average_execution_time = logariphmic_complexity::get_execution_time(counter, 1000);

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
        
        let average_execution_time = linear_complexity::get_execution_time(counter, 1000);

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
        
        let average_execution_time = o_n_log_n_complexity::get_execution_time(counter, 1000);

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
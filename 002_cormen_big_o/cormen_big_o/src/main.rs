mod constant_complexity;
mod logariphmic_complexity;
mod linear_complexity;
mod o_n_log_n_complexity;
mod square_complexity;
mod cube_complexity;
mod complexity_2_in_n;
mod factorial_complexity;
mod perfomance_test_data;
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Разработка и отладка примеров сложности Big_O");

    let perfomance_o_1 = bigo_complexity_o_1();
    let perfomance_o_log = bigo_complexity_o_log();
    let perfomance_o_linear = bigo_complexity_linear();
    let perfomance_o_n_log_n = bigo_complexity_n_log_n();
    let perfomance_o_n_square_2 = bigo_complexity_n_square_2();
    let perfomance_o_n_square_3 = bigo_complexity_n_square_3();
    let perfomance_o_n_square_n = bigo_complexity_2_square_n();
    let perfomance_o_n_factorial = bigo_complexity_factorial();

    let mut alg_min_time: f32 = 0.0;
    let mut alg_min_elements: f32 = 0.0;
    let mut alg_max_time: f32 = 0.0;
    let mut alg_max_elements: f32 = 0.0;

    for element in &perfomance_o_1 {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_log {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_linear {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_n_log_n {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_n_square_2 {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_n_square_3 {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_n_square_n {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    for element in &perfomance_o_n_factorial {
        if element.nanoseconds > alg_max_time { alg_max_time = element.nanoseconds;}
        if (element.data_count as f32) > alg_max_elements {alg_max_elements = (element.data_count as f32); }
    }

    //draw basic bitmap
    let root = BitMapBackend::new("./plotters-doc-data/complexities_giagram.png", (1500, 1500)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("algorithms complexity", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(alg_min_elements..alg_max_elements, alg_min_time..alg_max_time)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(
        LineSeries::new(
            perfomance_o_1.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &RED
        ))?
        .label("O(1)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_log.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &BLUE
        ))?
        .label("log(o)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_linear.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &GREEN
        ))?
        .label("o(n)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_n_log_n.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &RGBColor(250, 125, 125)
        ))?
        .label("log o log")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(250, 125, 125)));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_n_square_2.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &RGBColor(100, 200, 100)
        ))?
        .label("o(n^2)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(100, 200, 100)));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_n_square_3.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &MAGENTA
        ))?
        .label("o(n^3)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &MAGENTA));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_n_square_n.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &RGBColor(100, 200, 50)
        ))?
        .label("o(n^n)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RGBColor(100, 200, 50)));

    chart.draw_series(
        LineSeries::new(
            perfomance_o_n_factorial.iter().map(|x| (x.data_count as f32, x.nanoseconds as f32)),
            &BLACK
        ))?
        .label("o(n!)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLACK));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    root.present()?;

    Ok(())
}

fn bigo_complexity_o_1() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = constant_complexity::get_execution_time(counter, 1000);

        println!("Сложность O(1), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_o_log() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = logariphmic_complexity::get_execution_time(counter, 100);

        println!("Сложность Olog, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_linear() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = linear_complexity::get_execution_time(counter, 100);

        println!("Сложность линейная - O(n), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_n_log_n() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = o_n_log_n_complexity::get_execution_time(counter, 100);

        println!("Сложность линейнлогарфмическая - O(n log n), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_n_square_2() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = square_complexity::get_execution_time(counter, 10);

        println!("Сложность квадратичная - O(n^2), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_n_square_3() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 200;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = cube_complexity::get_execution_time(counter, 1);

        println!("Сложность кубическая - O(n^3), среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }
        
        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_2_square_n() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 25;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = complexity_2_in_n::get_execution_time(counter, 10);

        println!("Сложность экспоненциальная, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}

fn bigo_complexity_factorial() -> Vec<perfomance_test_data::PerfomanceTestData> {
    let mut counter = 1;
    let step = 1;
    let max = 8;

    let mut result: Vec<perfomance_test_data::PerfomanceTestData> = vec![];

    loop {

        if counter > max {
            break;
        }
        
        let average_execution_time = factorial_complexity::get_execution_time(counter, 10);

        println!("Сложность факториальная, среднее время выполнения по {} записям = {} наносекунд", counter, average_execution_time);

        if average_execution_time > 200.0 {
            break;
        }

        let perfomance = perfomance_test_data::PerfomanceTestData {
            data_count: counter as u32,
            nanoseconds: average_execution_time
        };

        result.push(perfomance);

        counter = counter + step;
    }

    return result;
}
use calculations::calc;

fn main() {
    // generate numbers
    let numbers = calc::generate_numbers();
    println!("Numbers are {:?}", numbers);

    // calculate average
    let average = calc::calc_average(&numbers);
    println!("Average is {}", average);

    // calculate median
    let median = calc::calc_median(&numbers);
    println!("Median is {}", median);

    // calculate mode
    let (mode, times) = calc::calc_mode(&numbers);
    println!("Mode is {}, appeared {} times", mode, times);
}


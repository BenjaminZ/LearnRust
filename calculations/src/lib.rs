pub mod calc{
    use std::collections::HashMap;
    use rand::Rng;

    pub fn calc_mode(numbers: &Vec<i32>) -> (i32, usize) {
        let mut map: HashMap<i32, usize> = HashMap::new();

        for number in numbers {
            let count = map.entry(*number).or_insert(0);
            *count += 1;
        }

        let (mut max_value, mut max_count) = (0, 0usize);
        for (value, count) in map.iter() {
            if *count > max_count {
                max_value = *value;
                max_count = *count;
            }
        }

        (max_value, max_count)
    }

    pub fn calc_median(numbers: &Vec<i32>) -> i32 {
        let mut to_sort = numbers.clone();
        to_sort.sort();

        let mut i = to_sort.len() / 2;
        if to_sort.len() % 2 == 0 {
            i -= 1;
        }

        to_sort[i]
    }

    pub fn calc_average(numbers: &Vec<i32>) -> f32 {
        let mut sum = 0;

        for number in numbers {
            sum += number;
        }

        sum as f32 / numbers.len() as f32
    }

    pub fn generate_numbers() -> Vec<i32> {
        let count = 10;
        let mut numbers = vec![0; count];
        for number in &mut numbers {
            *number = rand::thread_rng().gen_range(-100..100);
        }
        numbers
    }
}
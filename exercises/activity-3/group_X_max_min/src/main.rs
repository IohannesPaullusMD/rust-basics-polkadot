fn main() {
    let numbers: [i32; 5] = [20, 2, 16, 4, 5];
    println!("{:?}", numbers);
    println!("max:{}", find_maximum(&numbers));
    println!("min:{}", find_minimum(&numbers));
}

fn find_minimum(numbers: &[i32]) -> i32 {
    let mut min = numbers[0];

    for i in 1..numbers.len() {
        if numbers[i] < min {
            min = numbers[i];
        }
    }

    min
}

fn find_maximum(numbers: &[i32]) -> i32 {
    let mut max = numbers[0];
    
    for i in 1..numbers.len() {
        if numbers[i] > max {
            max = numbers[i];
        }
    }

    max
}

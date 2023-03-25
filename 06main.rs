fn sum_u32(numbers: &[u32]) -> Option<u32> {
    numbers.iter().fold(Some(0), |acc, &num| {
        acc.and_then(|sum| sum.checked_add(num))
    })
}

fn main() {
    let numbers = &[1, 2, 3, 4, 5];
    let result = sum_u32(numbers);

    match result {
        Some(sum) => println!("The sum is: {}", sum),
        None => println!("Overflow occurred."),
    }
}

fn sum_pos(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in v.iter().filter(|i| **i > 0) {
        sum += *i;
    }
    sum
}

fn main() {
    let numbers = vec![1, -1, 2, -2, 3, -3, 4, -4, 5, -5];
    println!("sum_pos(numbers) = {}", sum_pos(&numbers));
}

fn sum_vec(memo:Vec<i32>)->(i32, Vec<i32>){/*snipped*/}
fn main() {
    let v = vec![1, 2, 3];
    let (sum, v) = sum_vec(v);
    println!("sum = {}", sum);   
    let (sum, v) = sum_vec(v);
    println!("sum = {}", sum);
}
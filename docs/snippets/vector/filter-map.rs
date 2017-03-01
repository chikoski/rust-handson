let v = vec![1, 2, 3, 4, 5];
let result = v.iter().filter(|&n| n % 2 != 0).map(|n| n + 1);
for (index, value) in result.enumerate() {
    println!("result[{}]:{}", index, value);
}
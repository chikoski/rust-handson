let x = 5;
let label = match x {
    1 => "壱",
    2 => "弐",
    3 => "参",
    4 | 5 | 6 | 7 | 8 | 9 => "一桁",
    _ => "その他",
};
println!("{} : {}", x, label);
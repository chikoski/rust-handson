let x = 5;
let label = match x {
    1 => "壱",
    2 => "弐",
    3 => "参",
    _ => "その他"
};
println!("{} : {}", x, label);
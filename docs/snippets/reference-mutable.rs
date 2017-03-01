fn foo(v: &mut Vec<i32>){
    v.push(5);
}
fn main(){
    let mut v = vec![];
    println!("v.len() = {}", v.len());
    foo(&mut v);
    println!("v.len() = {}", v.len());    
} 
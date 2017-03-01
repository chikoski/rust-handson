fn foo(v: &Vec<i32>){
    v.push(5);
}
fn main(){
    let v = vec![];
    foo(&v);
}   
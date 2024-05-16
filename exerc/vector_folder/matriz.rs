fn main(){
    matriz();
}
fn matriz(){
    let v: Vec<i32> = Vec::new();

    let mut v = vec![1,2,3,4];
    v.push(11);
    println!("{:?}", v);
}
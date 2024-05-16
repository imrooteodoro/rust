fn main(){
    vec_func();
}

fn vec_func(){
    let mut i = vec![0, 5, 150];
    let mut j = vec![0, 10, 100];
    let mut iterator = 0;
    let mut result = 0; 
    while iterator <  i.len(){
         result = i[iterator] * j [iterator];
         iterator+=1;
    } 
    println!("{:?}", result);
}
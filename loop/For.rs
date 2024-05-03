fn main(){
    for_func();
}

fn for_func(){
    let a = vec![4];
    for i in &a{
        println!("Compiled {i}");
        
    }
}
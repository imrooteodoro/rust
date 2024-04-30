fn main(){
    ForFunc();
}

fn ForFunc(){
    let a = vec![4];
    for i in &a{
        println!("Compiled {i}");
        
    }
}
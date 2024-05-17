
fn main(){
    vec_func();

   
}

fn vec_func(){
    let i = vec![5, 10, 5, 7, 6];
    let mut result = vec![] ;
    let mut iterator = 0; 
    while iterator < i.len() {
       if i[iterator] % 2 != 0{
        result = vec![i[iterator]];
        
       }

       iterator+=1;
    } 
    println!("{:?}", result);
}

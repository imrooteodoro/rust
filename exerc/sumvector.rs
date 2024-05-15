fn main(){
   sumvec();
}

fn sumvec(){
    let mut x = vec![10,30,40,10];
    let mut y = vec![20,60,70,23];
    let mut i = 0;
    let mut result = 0;

    while i < x.len(){
         result = x[i] - y[i] ;
        i+=1;
    }
    println!("{}", result);
}
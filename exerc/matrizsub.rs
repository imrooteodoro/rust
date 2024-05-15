fn main(){
    vec_sub();
}

fn vec_sub(){
    let mut x = vec![2,7];
    let mut i = 0;
    while i < x.len(){
        println!("{:?}", x[i] * 2);

        i+=1;
    }
}
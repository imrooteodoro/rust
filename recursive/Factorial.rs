fn main(){
    println!("O fatorial de n é {}", fac(5));
}

fn fac(n:i32)->i32{
    if n <=1{
        return 1;
    }
    else{
        return n * fac(n-1);
    }
}
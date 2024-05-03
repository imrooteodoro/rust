fn main(){
    println!("{}", fac_match(5));
}

fn fac_match(n:i32) -> i32{
    if n < 0{
        return 1;
    }
    else{
        match n{
            0|1=> 1,
            _=>return n* fac_match(n-1),
        }
    } 
}
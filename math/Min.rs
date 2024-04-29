fn main(){
    println!("{}", min(4, 6))
}

fn min(x:i8, y:i8) -> i8{
    match x{
         4=> x - y * 2,
        _=> x - y,
    }
}
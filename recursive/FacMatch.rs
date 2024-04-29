fn main(){
    println!("{}", facMatch(5));
}

fn facMatch(n:i32) -> i32{
    if n < 0{
        return 1;
    }
    else{
        match n{
            0|1=> 1,
            _=>return n* facMatch(n-1),
        }
    } 
}
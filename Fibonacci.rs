fn main(){
    print!("{}", fib(10))
}
fn fib(n:u32) -> u32{
    match n{
        0=>1,
        1=>1,
        2=>1,
        _=>fib(n-1) + fib(n-2),
    }
}

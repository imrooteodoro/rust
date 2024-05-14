fn main(){
    println!("{}", exp(3, 2));
}

fn exp(n:u32, i:u32) -> u32{
    return n.pow(i) as u32;
}
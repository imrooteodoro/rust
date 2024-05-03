fn main(){
    println!("O número é primo? {}",prime(5));
}
fn prime(n:i32)->bool{
    if n < 1{
      return false;
    }
    let sqrt_n = (n as f32).sqrt() as i32;
    for i in 2..=sqrt_n {
        if n% i == 0{
            return false;
        }
    }
    true

}
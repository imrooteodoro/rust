fn main(){
    println!("{}", x_if(6))
}
fn x_if(y:i8)->i8{
    if y > 5 && y % 2 ==0{
        return y;
    }else{
        return 0;
    }

}
fn main(){
    println!("{}", xIf(6))
}
fn xIf(y:i8)->i8{
    if y > 5 && y % 2 ==0{
        return y;
    }else{
        return 0;
    }

}
fn main(){
    println!("____________________\n
    
             Hello!\n
    __________________________\n");
    println!(" X: {}", SumIf(4))
    
}

fn SumIf(x:u8) -> u8{
    if x > 5{
        return x + 1;
    }else{
         return 0;
    }
}

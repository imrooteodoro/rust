fn main(){
    sum_vector();
}

fn sum_vector(){
    let mut v = vec![0, 3, 4, 5];
    let mut x = vec![1, 4, 6, 8];
    let mut index = 0;
    while index < 4{
        println!("{:?}", v[index] * x[index]);
        index +=1;
    }

}
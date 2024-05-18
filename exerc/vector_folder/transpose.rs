fn main(){
    transpose();
}
fn transpose(){
    let x: [[i32;3]; 2] = [[1, 5, 6], [1,4,5]];

    let row = x.len();
    let col = x[0].len();

let mut transposed:Vec<Vec<i32>> =vec![vec![0;row];col];
let mut y : Vec<Vec<i32>> = vec![vec![0;row];col];

for i in 0..row{
    for j in 0..col{
        transposed[j][i] = x[i][j];
        y[j][i] = transposed[j][i] * transposed[j][i];
    }
}
println!("{:?}", y);
println!("{:?}", x);
println!("{:?}", transposed);
}
fn main(){
    vector();
}

fn vector(){
    let x: [[i32;3];3] = [[1,2,3], [1,2,3], [1,2,3]];
    let  row = x.len();
    let  col = x[0].len();
    let mut transposed: Vec<Vec<i32>> = vec![vec![0; row]; col];

    for i in 0..row{
        for j in 0..col{
            transposed[j][i] = x[i][j];
        }
    }

    println!("{:?}", transposed);
}

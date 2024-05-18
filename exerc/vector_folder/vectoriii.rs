fn main(){
    transpose();
}

fn transpose(){
   
    let x:[[i32;3];2] = [[1, 4, 6], [2, 4, 6]];
    let rowx = x.len();
    let colx = x[0].len();
    let y:[[i32; 2];3] = [[1,3], [5, 6], [6, -8]];
    let mut xt:Vec<Vec<i32>> = vec![vec![0;rowx];colx];

    for i in 0..rowx{
        for j in 0..colx{
            xt[j][i] = x[i][j];
            xt[j][i] -= y[j][i];

        }
    }
    println!("{:?}", y);
    println!("{:?}",xt );

}
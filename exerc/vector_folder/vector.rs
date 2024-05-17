
fn main(){
    vec_func();
    mult();
    matrix();

   
}

fn vec_func(){
    let i = vec![5, 10, 5, 7, 6, 11];
    let mut result = vec![] ;
    let mut iterator = 0; 
    while iterator < i.len() {
       if i[iterator] % 2 != 0{
        result.push(i[iterator]);
        
       }

       iterator+=1;
    } 
    println!("{:?}", result);
}

fn mult(){
    let mut i = 0;
    let y = vec![5,5,5];
    let mut res = vec![];
    while i < y.len(){
        res.push(y[i] * y[i]);
        i+=1;
    }
    println!("{:?}", res);
}

fn matrix(){
    let matrix: [[i32; 3]; 2] = [[1, 1, 1], [1, 1, 1]];
    let matrixI: [[i32; 3]; 2] = [[2, 2, 2], [2, 2, 2]];
    let mut res: [[i32; 3]; 2] = [[0; 3]; 2];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            res[i][j] = matrix[i][j] * matrixI[i][j];
        }
    }
    
    println!("{:?}", res);
}



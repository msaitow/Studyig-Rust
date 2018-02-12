
fn main(){
    let x = [0,1,2,3,4];
    let m = &x[1..4];
    println!("m[0] = {}", m[0]);
    println!("m[1] = {}", m[1]);
    println!("m[2] = {}", m[2]);
    println!("m = {}", m.len());
    
    println!("x[0] = {}", x[0]);
    println!("x.len() = {}", x.len());
}

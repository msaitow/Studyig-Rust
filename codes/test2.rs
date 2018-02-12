
fn main(){
    let x : i32 = 17;
    {
        let y : i32 = 3;
        println!("@1 x, y = {}, {}", x, y); 
    }
    //println!("@2 x, y = {}, {}", x, y);
}

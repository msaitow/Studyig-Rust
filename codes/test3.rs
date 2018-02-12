
fn addOne (x : i32) -> i32 {
    x + 1
}

fn main(){
    //let f: fn(i32) -> i32;
    let f = addOne;
    
    let x : i32 = 8;
    {
        println!("@1 x = {}", x); 
        let x : i32 = 3;
        println!("@2 x = {}", x); 
    }
    let x : i32 = 2;
    println!("@3 x = {}", x);
    println!("@4 x = {}", addOne(x));
    println!("@4 x = {}", f(x));
}

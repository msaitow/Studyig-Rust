
fn main(){
    'outer: for x in 0..10 {
        'inner: for y in 0..10 {
            if x % 2 == 0 { continue 'outer; } // x のループを継続
            if y % 2 == 0 { continue 'inner; } // y のループを継続
            println!("x: {}, y: {}", x, y);
        }
    }
}
